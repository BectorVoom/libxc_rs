//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta266(t1420: f64, t2453: f64, t3908: f64, t1426: f64, t786: f64, t64: f64, t843: f64, t112: f64, t2289: f64, t666: f64, t654: f64, t98: f64, t99: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10165, t10166, t10174, t10175, t10199, t10201, t10202, t10207, t10208, t10226) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1476(t1420, t2453, t3908, t1426, t786, t64, t843, t112, t2289, t666, t654, t98, t99);
    (t10165, t10166, t10174, t10175, t10199, t10201, t10202, t10207, t10208, t10226)
}
