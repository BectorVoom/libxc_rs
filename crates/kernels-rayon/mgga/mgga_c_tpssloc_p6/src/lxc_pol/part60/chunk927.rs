//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 927/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk927(t22642: f64, t22690: f64, t31193: f64, t22716: f64, t8480: f64, t22724: f64, t31198: f64, t2006: f64, t794: f64, t31127: f64, t31104: f64, t8455: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114064 = 0.16449340668482264365e-1_f64 * t22642 * t22690 * t31193;
    let t114104 = 0.12793931631041761173e0_f64 * t22716 * t8480;
    let t114119 = 0.52089578783527170489e-1_f64 * t22724 * t31198;
    let t114172 = t794 * t2006;
    let t114178 = 0.52089578783527170489e-1_f64 * t22724 * t31127;
    let t114225 = 0.52089578783527170489e-1_f64 * t22724 * t31104;
    let t114264 = 0.12793931631041761173e0_f64 * t22716 * t8455;
    (t114064, t114104, t114119, t114172, t114178, t114225, t114264)
}
