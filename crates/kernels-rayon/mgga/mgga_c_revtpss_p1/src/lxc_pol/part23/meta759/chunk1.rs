//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2553/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2553(t15749: f64, t3211: f64, t16199: f64, t372: f64, t16208: f64, t1025: f64, t1663: f64, t2434: f64, t371: f64, t225: f64, t53166: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54648 = t3211 * t15749;
    let t54658 = t372 * t16199;
    let t54672 = t372 * t16208;
    let t54687 = t1025 * t371 * t2434 * t1663;
    let t54695 = t53166 * t225;
    let t54696 = t54695 * t366;
    (t54648, t54658, t54672, t54687, t54695, t54696)
}
