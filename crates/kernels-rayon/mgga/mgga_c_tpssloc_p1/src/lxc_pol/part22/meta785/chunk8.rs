//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2711/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2711(t16: f64, t39031: f64, t12774: f64, t19503: f64, t2: f64, t20311: f64, t20312: f64, t20315: f64, t20318: f64, t20319: f64, t20322: f64, t2219: f64, t2341: f64, t4049: f64, t4060: f64, t45496: f64, t45697: f64, t5396: f64, t5468: f64, t5475: f64, t584: f64, t657: f64, t659: f64, t663: f64, t75631: f64, t92: f64, t95: f64) -> (f64, f64) {
    let t75649 = 6.0_f64 * t16 + 12.0_f64 * t39031;
    let t75657 = 50.0_f64 / 81.0_f64 * t657 * t20312 + 40.0_f64 / 81.0_f64 * t92 * t45496 * t20311 * t659 - 10.0_f64 / 9.0_f64 * t45697 * t5468 * t2 * t584 - 50.0_f64 / 9.0_f64 * t657 * t20315 - 10.0_f64 / 9.0_f64 * t45697 * t75631 * t659 + 10.0_f64 / 3.0_f64 * t12774 * t2219 * t5396 + 10.0_f64 / 3.0_f64 * t92 * t4049 * t19503 - 25.0_f64 / 9.0_f64 * t657 * t20319 + 10.0_f64 / 9.0_f64 * t92 * t2341 * t20318 * t659 + 5.0_f64 / 3.0_f64 * t92 * t95 * t75649 - 2200.0_f64 / 81.0_f64 * t20322 * t663 + 400.0_f64 / 27.0_f64 * t5475 * t4060;
    (t75649, t75657)
}
