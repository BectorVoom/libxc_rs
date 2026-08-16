//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1995;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta583(t2482: f64, t596: f64, t7043: f64, t2677: f64, t240: f64, t25260: f64, t25228: f64, t9775: f64, t10073: f64, t25308: f64, t25403: f64, t25402: f64, t7048: f64, t7056: f64, t233: f64, t41077: f64, t9646: f64, t1949: f64, t22: f64, t1954: f64, t39643: f64, t2470: f64, t25295: f64, t7058: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93072, t93073, t93082, t93088, t93112, t93116) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1995(t2482, t596, t7043, t2677, t240, t25260, t25228, t9775, t10073, t25308, t25403, t25402, t7048, t7056);
        let (t93118, t93138, t93139, t93142, t93150, t93151) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1996(t233, t41077, t7056, t9646, t1949, t22, t25402, t1954, t39643, t2470, t25295, t7058);
    (t93072, t93073, t93082, t93088, t93112, t93116, t93118, t93138, t93139, t93142, t93150, t93151)
}
