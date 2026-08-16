//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1346/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1346(t32762: f64, t6883: f64, t1985: f64, t214: f64, t225: f64, t26328: f64, t567: f64, t7722: f64, t6907: f64, t120334: f64, t120337: f64, t120340: f64, t120425: f64, t120436: f64, t120488: f64, t120528: f64, t1375: f64, t1378: f64, t1807: f64, t22656: f64, t26366: f64, t31117: f64, t31181: f64, t31217: f64, t32766: f64, t3882: f64, t5321: f64, t539: f64, t568: f64, t6963: f64, t7750: f64) -> (f64, f64) {
    let t120532 = t6883 * t32762;
    let t120533 = 0.38381794893125283518e-1_f64 * t120532;
    let t120542 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t26328 * t225 * t567;
    let t120544 = t214 * t7722;
    let t120547 = 0.16449340668482264365e-1_f64 * t1985 * t120544 * t6907;
    let t120548 = t120334 - t120337 - t120340 + t539 * t120425 * t568 + 4.0_f64 * t26366 * t6963 + 4.0_f64 * t3882 * t32766 - 6.0_f64 * t5321 * t31117 - t120436 - t1375 * t1378 * (t120488 + t120528) - t120533 + t1807 * t31181 * t568 - 2.0_f64 * t22656 * t7750 + t120542 - t5321 * t31217 - t120547;
    (t120544, t120548)
}
