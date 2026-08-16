//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1978;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta567(t10309: f64, t25105: f64, t45972: f64, t6957: f64, t1962: f64, t41154: f64, t2411: f64, t605: f64, t198: f64, t206: f64, t7086: f64, t25373: f64, t25392: f64, t25386: f64, t25372: f64, t2435: f64, t25352: f64, t11015: f64, t7018: f64, t7048: f64, t822: f64, t25300: f64, t9285: f64, t25299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92687, t92690, t92742, t92790, t92819, t92837) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1978(t10309, t25105, t45972, t6957, t1962, t41154, t2411, t605, t198, t206, t7086, t25373, t25392);
        let (t92838, t92843, t92858, t92861, t92864, t92868, t92870) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1979(t25386, t92837, t25372, t2435, t25352, t11015, t7018, t7048, t822, t25300, t9285, t25299);
    (t92687, t92690, t92742, t92790, t92819, t92838, t92843, t92858, t92861, t92864, t92868, t92870)
}
