//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2002;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta552(t10301: f64, t25105: f64, t116: f64, t25168: f64, t1962: f64, t41154: f64, t2411: f64, t25435: f64, t605: f64, t198: f64, t206: f64, t7086: f64, t25373: f64, t25392: f64, t25386: f64, t268: f64, t41040: f64, t837: f64, t25372: f64, t25287: f64, t786: f64, t789: f64, t2829: f64, t689: f64, t7014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92702, t92737, t92742, t92775, t92790, t92819) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2002(t10301, t25105, t116, t25168, t1962, t41154, t2411, t25435, t605, t198, t206, t7086);
        let (t92838, t92841, t92843, t92844, t92847, t92856) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2003(t25373, t25392, t25386, t268, t41040, t837, t25372, t25287, t786, t789, t2829, t689, t7014);
    (t92702, t92737, t92742, t92775, t92790, t92819, t92838, t92841, t92843, t92844, t92847, t92856)
}
