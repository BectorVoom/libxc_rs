//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk924;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk925;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta195(t240: f64, t9948: f64, t247: f64, t550: f64, t548: f64, t4010: f64, t72: f64, t245: f64, t1386: f64, t820: f64, t844: f64, t2482: f64, t596: f64, t1384: f64, t235: f64, t239: f64, t4003: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9949, t9953, t9954, t9955, t9962) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk924(t240, t9948, t247, t550, t548, t4010, t72, t245, t1386, t820, t844);
        let (t9976, t9989, t9990, t9991) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk925(t1386, t2482, t596, t1384, t235);
        let (t9993, t9994) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk926(t239, t820, t9991, t4003, t543);
    (t9949, t9953, t9954, t9955, t9962, t9976, t9989, t9990, t9991, t9993, t9994)
}
