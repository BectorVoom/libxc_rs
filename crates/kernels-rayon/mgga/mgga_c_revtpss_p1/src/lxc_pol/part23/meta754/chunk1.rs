//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2544/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2544(t247: f64, t42792: f64, t4757: f64, t4837: f64, t3091: f64, t43240: f64, t4782: f64, t41296: f64, t42471: f64, t3155: f64, t999: f64, t1011: f64, t4886: f64, t697: f64) -> (f64, f64, f64, f64, f64) {
    let t53431 = t4837 * t247 * t42792 * t4757;
    let t53432 = 0.28582678745379824648e-3_f64 * t53431;
    let t53437 = t3091 * t43240 * t4782;
    let t53473 = t42471 * t41296;
    let t53511 = t3155 * t999;
    let t53542 = t1011 * t697 * t4886;
    (t53432, t53437, t53473, t53511, t53542)
}
