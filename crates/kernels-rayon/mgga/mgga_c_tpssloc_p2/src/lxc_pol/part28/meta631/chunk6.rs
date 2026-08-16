//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1983/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1983(t87432: f64, t87443: f64, t81918: f64, t81924: f64, t81926: f64, t81928: f64, t81934: f64, t81936: f64, t81943: f64, t84921: f64, t87418: f64, t87422: f64, t87425: f64, t87428: f64, t87430: f64, t87445: f64, t87449: f64, t87453: f64) -> f64 {
    let t92689 = 0.22608743412718618878e-1_f64 * t87432;
    let t92697 = 0.80745512188280781706e-3_f64 * t87443;
    let t92701 = 0.33913115119077928316e-1_f64 * t87418 - t87422 / 2.0_f64 - 0.23739180583354549822e0_f64 * t87425 + 0.16956557559538964158e-1_f64 * t87428 - t87430 / 24.0_f64 - t92689 - 0.13457585364713463618e-3_f64 * t81918 - t84921 + 0.67287926823567318088e-4_f64 * t81924 - 7.0_f64 / 1152.0_f64 * t81926 + 119.0_f64 / 1728.0_f64 * t81928 - 0.27130492095262342653e0_f64 * t81934 + 0.16956557559538964158e-1_f64 * t81936 - 35.0_f64 / 54.0_f64 * t81943 + t92697 + 0.20186378047070195426e-3_f64 * t87445 - 0.33913115119077928316e-1_f64 * t87449 + 0.48447307312968469024e-2_f64 * t87453;
    t92701
}
