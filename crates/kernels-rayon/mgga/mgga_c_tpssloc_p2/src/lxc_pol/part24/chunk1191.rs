//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1191/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1191(t1933: f64, t607: f64, t1937: f64, t1000: f64, t1025: f64, t23414: f64, t23419: f64, t23422: f64, t23425: f64, t23433: f64, t23437: f64, t3073: f64, t3098: f64, t3123: f64, t3143: f64, t3148: f64, t6717: f64, t6755: f64, t6765: f64) -> f64 {
    let t23442 = t1933 * t607;
    let t23443 = t23442 * t1937;
    let t23445 = 0.10093189023535097714e-3_f64 * t23414 * t1937 + t23419 * t3073 / 1152.0_f64 - t23422 * t1000 / 54.0_f64 + t23425 / 432.0_f64 + t6717 * t3143 / 288.0_f64 + t6717 * t3148 / 216.0_f64 + t6755 * t3123 / 1536.0_f64 + t23433 * t1025 / 768.0_f64 - t23437 * t1025 / 144.0_f64 - t6765 * t3098 / 1152.0_f64 + 0.20186378047070195428e-3_f64 * t23443;
    t23445
}
