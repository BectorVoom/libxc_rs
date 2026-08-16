//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2294/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2294(t1933: f64, t23479: f64, t99660: f64, t1015: f64, t28581: f64, t82895: f64, t28577: f64, t3128: f64, t25641: f64, t88451: f64, t1615: f64, t17157: f64, t17167: f64, t17171: f64, t1920: f64, t25679: f64, t25683: f64, t2987: f64, t363: f64, t4509: f64, t6800: f64, t88351: f64, t88354: f64, t88372: f64, t88430: f64, t88431: f64, t88704: f64) -> f64 {
    let t99796 = t1933 * t99660 * t23479;
    let t99799 = t82895 * t1015 * t28581;
    let t99802 = t82895 * t3128 * t28577;
    let t99813 = t88451 * t25641;
    let t99826 = t88704 - 0.20186378047070195428e-3_f64 * t99796 - 0.10093189023535097714e-3_f64 * t99799 + 0.20186378047070195428e-3_f64 * t99802 - 0.20186378047070195428e-3_f64 * t88430 * t88431 * t363 * t1615 * t6800 - 0.40372756094140390856e-3_f64 * t88372 * t88351 + 0.20186378047070195428e-3_f64 * t88372 * t88354 + 0.20186378047070195428e-3_f64 * t99813 - t1920 * t2987 * t17171 / 72.0_f64 - t1920 * t4509 * t17157 / 36.0_f64 + t1920 * t2987 * t17167 / 48.0_f64 + 0.20186378047070195428e-3_f64 * t25683 * t25679;
    t99826
}
