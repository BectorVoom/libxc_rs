//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 734/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk734<F: Float>(t11385: F, t1887: F, t706: F, t1417: F, t4660: F, t11325: F, t11330: F, t11335: F, t11338: F, t11340: F, t11342: F, t11344: F, t11347: F, t11350: F, t1421: F, t456: F) -> (F, F) {
    let t11386 = t1887 * t11385;
    let t11387 = t706 * t11386;
    let t11390 = t1417 * t4660;
    let t11392 = F::cast_from(0.39422577999999999999e-2_f64) * t1421 * t11325 - F::cast_from(0.65704296666666666666e-2_f64) * t1421 * t11330 + F::cast_from(0.22175200125e-2_f64) * t1421 * t11335 - F::new(0.19711289e-2) * t11338 + F::cast_from(0.1478346675e-2_f64) * t11340 + F::cast_from(0.295669335e-2_f64) * t11342 + F::cast_from(0.65704296666666666665e-3_f64) * t11344 + F::cast_from(0.1478346675e-2_f64) * t456 * t11347 - F::new(0.98556445e-3) * t11350 - F::new(0.98556445e-3) * t456 * t11387 + F::cast_from(0.39422577999999999999e-2_f64) * t11390;
    (t11386, t11392)
}
