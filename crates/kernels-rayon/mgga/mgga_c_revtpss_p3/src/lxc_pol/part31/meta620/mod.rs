//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2069;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta620(t25301: f64, t99257: f64, t25410: f64, t7774: f64, t93240: f64, t1032: f64, t4469: f64, t867: f64, t786: f64, t7060: f64, t7760: f64, t2467: f64, t10073: f64, t25403: f64, t27198: f64, t1955: f64, t2471: f64, t27202: f64, t15003: f64, t93194: f64, t27266: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99258, t99261, t99270, t99271, t99274, t99285, t99287) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2069(t25301, t99257, t25410, t7774, t93240, t1032, t4469, t867, t786, t7060, t7760, t2467);
        let (t99297, t99303, t99307, t99313, t99321) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2070(t10073, t25403, t27198, t1955, t99270, t2471, t27202, t15003, t93194, t27266, t686, t72);
    (t99258, t99261, t99271, t99274, t99285, t99287, t99297, t99303, t99307, t99313, t99321)
}
