//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1665;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta355(t2710: f64, t2713: f64, t4371: f64, t4353: f64, t808: f64, t10744: f64, t10905: f64, t4442: f64, t240: f64, t849: f64, t14648: f64, t775: f64, t2661: f64, t2652: f64, t4345: f64, t10716: f64, t4349: f64, t2689: f64, t4372: f64, t4354: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14817, t14819, t14820, t14823, t14832) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1665(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t240, t849);
        let (t14833, t14834, t14836, t14837, t14839, t14846, t14850) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1666(t14648, t775, t14832, t2661, t2652, t4345, t10716, t4349, t2689, t4372, t4354, t9775);
    (t14817, t14819, t14820, t14823, t14832, t14833, t14834, t14836, t14837, t14839, t14846, t14850)
}
