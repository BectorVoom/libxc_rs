//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 798/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk798(t12225: f64, t12318: f64, t2029: f64, t1990: f64, t5444: f64, t11222: f64, t11231: f64, t11233: f64, t11239: f64, t11241: f64, t11453: f64, t11456: f64, t11650: f64, t11652: f64, t11661: f64, t11663: f64, t11669: f64, t11674: f64, t11680: f64, t11685: f64, t11967: f64, t1994: f64, t5440: f64) -> f64 {
    let t12319 = t12225 + t12318;
    let t12320 = t12319 * t2029;
    let t12325 = t1990 * t5444;
    let t12338 = 0.38691203703703703703e-2_f64 * t11222 - 0.10446625e-1_f64 * t11231 - 0.77382407407407407405e-3_f64 * t11233 + 0.10446625e-1_f64 * t11239 + 0.69644166666666666665e-2_f64 * t11241 - 0.193e0_f64 * t1994 * t12320 - 0.386e0_f64 * t1994 * t11967 + 0.223494e0_f64 * t12325 * t5440 - 0.17411041666666666666e-2_f64 * t11453 + 0.34822083333333333333e-2_f64 * t11456 + 0.17411041666666666666e-2_f64 * t11650 + 0.34822083333333333333e-2_f64 * t11652 - 0.52233124999999999998e-2_f64 * t11661 - 0.69644166666666666665e-2_f64 * t11663 - 0.69644166666666666665e-2_f64 * t11669 + 0.10446625e-1_f64 * t11674 - 0.77382407407407407405e-3_f64 * t11680 - 0.46429444444444444443e-2_f64 * t11685;
    t12338
}
