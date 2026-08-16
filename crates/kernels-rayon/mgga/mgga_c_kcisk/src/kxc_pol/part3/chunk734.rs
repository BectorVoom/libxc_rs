//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 734/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk734(t11385: f64, t1887: f64, t706: f64, t1417: f64, t4660: f64, t11325: f64, t11330: f64, t11335: f64, t11338: f64, t11340: f64, t11342: f64, t11344: f64, t11347: f64, t11350: f64, t1421: f64, t456: f64) -> (f64, f64) {
    let t11386 = t1887 * t11385;
    let t11387 = t706 * t11386;
    let t11390 = t1417 * t4660;
    let t11392 = 0.39422577999999999999e-2_f64 * t1421 * t11325 - 0.65704296666666666666e-2_f64 * t1421 * t11330 + 0.22175200125e-2_f64 * t1421 * t11335 - 0.19711289e-2_f64 * t11338 + 0.1478346675e-2_f64 * t11340 + 0.295669335e-2_f64 * t11342 + 0.65704296666666666665e-3_f64 * t11344 + 0.1478346675e-2_f64 * t456 * t11347 - 0.98556445e-3_f64 * t11350 - 0.98556445e-3_f64 * t456 * t11387 + 0.39422577999999999999e-2_f64 * t11390;
    (t11386, t11392)
}
