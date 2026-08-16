//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 994/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk994<F: Float>(t1556: F, t4495: F, t1553: F, t4346: F, t13291: F, t13297: F, t13302: F, t13307: F, t13309: F, t13313: F, t13318: F, t13323: F, t13325: F, t13334: F, t1598: F, t4351: F) -> F {
    let t14636 = t4495 * t1556;
    let t14639 = t1553 * t4346;
    let t14645 = -F::cast_from(0.17411041666666666666e-2_f64) * t13291 - F::cast_from(0.46429444444444444443e-2_f64) * t13297 + F::cast_from(0.69644166666666666666e-2_f64) * t13302 - F::cast_from(0.69644166666666666665e-2_f64) * t13307 + F::cast_from(0.46429444444444444443e-2_f64) * t13309 - F::cast_from(0.12381185185185185185e-1_f64) * t13313 + F::cast_from(0.23214722222222222222e-2_f64) * t13318 - F::cast_from(0.579e0_f64) * t14636 * t1598 + F::cast_from(0.223494e0_f64) * t14639 * t4351 - F::cast_from(0.34822083333333333333e-2_f64) * t13323 - F::cast_from(0.77382407407407407405e-3_f64) * t13325 - F::cast_from(0.10446625e-1_f64) * t13334;
    t14645
}
