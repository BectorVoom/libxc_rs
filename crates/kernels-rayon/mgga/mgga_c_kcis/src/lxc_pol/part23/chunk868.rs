//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 868/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk868(t16355: f64, t16369: f64, t1444: f64, t1482: f64, t16360: f64, t1102: f64, t11632: f64, t15991: f64, t15994: f64, t15997: f64, t16001: f64, t16003: f64, t16006: f64, t16010: f64, t16014: f64, t16018: f64, t16022: f64, t16026: f64, t16031: f64, t16035: f64, t16038: f64, t16349: f64, t16353: f64, t16356: f64, t16361: f64, t16366: f64, t486: f64) -> f64 {
    let t16370 = t16369 * t16355;
    let t16373 = t1482 * t1444;
    let t16374 = t16373 * t16360;
    let t16377 = 0.19711289e-2_f64 * t1102 * t15991 + 0.21901432222222222221e-2_f64 * t15994 - 0.7391733375e-3_f64 * t1102 * t15997 + t16001 - t16003 + 0.1478346675e-2_f64 * t1102 * t16006 + 0.7391733375e-3_f64 * t1102 * t16010 - 0.19711289e-2_f64 * t11632 * t16014 + 0.26281718666666666666e-2_f64 * t11632 * t16018 + 0.98556445e-3_f64 * t11632 * t16022 - 0.19711289e-2_f64 * t11632 * t16026 - 0.295669335e-2_f64 * t1102 * t16031 - 0.1478346675e-2_f64 * t1102 * t16035 - 0.14600954814814814815e-3_f64 * t16038 - 4.0_f64 * t486 * t16349 + 0.32852148333333333333e-2_f64 * t16353 * t16356 - 0.21901432222222222222e-2_f64 * t16353 * t16361 - 0.19711289e-2_f64 * t11632 * t16366 - 0.39422578e-2_f64 * t11632 * t16370 + 0.26281718666666666666e-2_f64 * t11632 * t16374;
    t16377
}
