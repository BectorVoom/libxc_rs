//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 994/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk994(t1556: f64, t4495: f64, t1553: f64, t4346: f64, t13291: f64, t13297: f64, t13302: f64, t13307: f64, t13309: f64, t13313: f64, t13318: f64, t13323: f64, t13325: f64, t13334: f64, t1598: f64, t4351: f64) -> f64 {
    let t14636 = t4495 * t1556;
    let t14639 = t1553 * t4346;
    let t14645 = -0.17411041666666666666e-2_f64 * t13291 - 0.46429444444444444443e-2_f64 * t13297 + 0.69644166666666666666e-2_f64 * t13302 - 0.69644166666666666665e-2_f64 * t13307 + 0.46429444444444444443e-2_f64 * t13309 - 0.12381185185185185185e-1_f64 * t13313 + 0.23214722222222222222e-2_f64 * t13318 - 0.579e0_f64 * t14636 * t1598 + 0.223494e0_f64 * t14639 * t4351 - 0.34822083333333333333e-2_f64 * t13323 - 0.77382407407407407405e-3_f64 * t13325 - 0.10446625e-1_f64 * t13334;
    t14645
}
