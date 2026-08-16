//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2485/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2485(t17659: f64, t4644: f64, t10422: f64, t21573: f64, t3070: f64, t10390: f64, t10408: f64, t10937: f64, t14080: f64, t21516: f64, t21520: f64, t21574: f64, t3117: f64, t4337: f64, t49994: f64, t50048: f64, t5857: f64, t62441: f64, t62445: f64, t70442: f64) -> f64 {
    let t70711 = t4644 * t17659;
    let t70724 = t3070 * t10422 * t21573;
    let t70728 = -t49994 - t14080 * t5857 / 288.0_f64 + t70711 / 2304.0_f64 + t62441 / 216.0_f64 - 5.0_f64 / 20736.0_f64 * t62445 + 5.0_f64 / 2304.0_f64 * t3070 * t10408 * t4337 * t70442 - t10390 * t21520 / 768.0_f64 - t10937 * t21574 / 288.0_f64 + t70724 / 2304.0_f64 + 5.0_f64 / 5184.0_f64 * t3117 * t21516 + t50048;
    t70728
}
