//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1961;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta536(t29135: f64, t7642: f64, t2148: f64, t1294: f64, t8208: f64, t26969: f64, t1775: f64, t1829: f64, t2149: f64, t2152: f64, t27008: f64, t27011: f64, t27025: f64, t29111: f64, t29119: f64, t29124: f64, t29129: f64, t29132: f64, t5246: f64, t7602: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t7654: f64, t7659: f64, t7662: f64, t7666: f64, t8198: f64, t8205: f64, t8217: f64, t1794: f64, t2142: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t29136, t29141, t29148, t29149, t29154) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1961(t29135, t7642, t2148, t1294, t8208, t26969, t1775, t1829, t2149, t2152, t27008, t27011, t27025, t29111, t29119, t29124, t29129, t29132, t5246, t7602, t7643, t7645, t7648, t7651, t7654, t7659, t7662, t7666, t8198, t8205, t8217);
        let (t29157, t29158) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1962(t1794, t2142, t73);
    (t29136, t29141, t29148, t29149, t29154, t29157, t29158)
}
