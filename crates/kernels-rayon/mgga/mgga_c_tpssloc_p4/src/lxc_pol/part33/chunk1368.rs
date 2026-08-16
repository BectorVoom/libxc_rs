//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1368/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1368(t106731: f64, t1874: f64, t20347: f64, t89: f64, t28030: f64, t7461: f64, t20563: f64, t24995: f64, t8945: f64, t1983: f64, t28238: f64, t5161: f64) -> (f64, f64, f64, f64, f64) {
    let t106733 = 6.0_f64 * t106731 * t1874;
    let t106734 = t89 * t20347;
    let t106736 = 2.0_f64 * t106734 * t1874;
    let t106738 = 6.0_f64 * t28030 * t7461;
    let t106741 = 18.0_f64 * t24995 * t8945 * t20563;
    let t106744 = 3.0_f64 * t1983 * t28238 * t5161;
    (t106733, t106736, t106738, t106741, t106744)
}
