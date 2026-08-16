//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2105;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta654(t2143: f64, t3566: f64, t17306: f64, t2142: f64, t3556: f64, t8945: f64, t12640: f64, t7635: f64, t29313: f64, t3801: f64, t12587: f64, t8220: f64, t29468: f64, t575: f64, t1464: f64, t8240: f64, t1921: f64, t7690: f64, t2167: f64, t5808: f64, t2172: f64, t5789: f64, t1913: f64, t7700: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t105576, t105579, t105598, t105644, t105665, t105669) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2105(t2143, t3566, t17306, t2142, t3556, t8945, t12640, t7635, t29313, t3801, t12587, t8220);
        let (t105792, t105794, t105796, t105798, t105800, t105802) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2106(t29468, t575, t1464, t8240, t1921, t7690, t2167, t5808, t2172, t5789, t1913, t7700);
    (t105576, t105579, t105598, t105644, t105665, t105669, t105792, t105794, t105796, t105798, t105800, t105802)
}
