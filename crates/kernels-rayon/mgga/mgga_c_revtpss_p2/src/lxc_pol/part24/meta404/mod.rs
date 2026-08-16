//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1341;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1342;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta404(t2783: f64, t9801: f64, t2735: f64, t4503: f64, t2682: f64, t820: f64, t823: f64, t10292: f64, t65: f64, t235: f64, t2710: f64, t826: f64, t225: f64, t785: f64, t2737: f64, t853: f64, t9794: f64, t66: f64, t240: f64, t10688: f64, t243: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40517, t40521, t40593, t40603, t40604, t40607) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1341(t2783, t9801, t2735, t4503, t2682, t820, t823, t10292, t65, t235, t2710, t826);
        let (t40609, t40611, t40627, t40633, t40634, t40638) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1342(t225, t40603, t785, t2737, t853, t9794, t10292, t66, t240, t10688, t243, t268);
    (t40517, t40521, t40593, t40604, t40607, t40609, t40611, t40627, t40633, t40634, t40638)
}
