//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1504;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1505;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta415(t1913: f64, t8302: f64, t2192: f64, t5789: f64, t116890: f64, t117095: f64, t117369: f64, t117374: f64, t117720: f64, t117765: f64, t1458: f64, t1464: f64, t18178: f64, t1921: f64, t31088: f64, t31329: f64, t4154: f64, t4168: f64, t5790: f64, t8373: f64, t8389: f64, t2184: f64, t5808: f64, t31328: f64, t575: f64, t8283: f64, t1455: f64, t116899: f64, t117090: f64, t117097: f64, t117099: f64, t117713: f64, t1456: f64, t18217: f64, t1914: f64, t2185: f64, t3: f64, t31127: f64, t31377: f64, t8284: f64) -> f64 {
        let t117777 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1504(t1913, t8302, t2192, t5789, t116890, t117095, t117369, t117374, t117720, t117765, t1458, t1464, t18178, t1921, t31088, t31329, t4154, t4168, t5790, t8373, t8389);
        let t117796 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1505(t2184, t5808, t31328, t575, t1921, t8283, t1455, t8389, t116899, t117090, t117097, t117099, t117713, t1456, t18217, t1914, t2185, t3, t31127, t31377, t8284);
        let tv4rho3tau2 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1506(t117777, t117796);
    tv4rho3tau2
}
