//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta784 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2593;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta784(t10368: f64, t56: f64, t1518: f64, t670: f64, t1921: f64, t5789: f64, t1913: f64, t5808: f64, t22532: f64, t575: f64, t21661: f64, t602: f64, t2246: f64, t5812: f64, t1469: f64, t627: f64, t72: f64, t10605: f64, t18539: f64, t11064: f64, t6075: f64, t37: f64, t5940: f64, t2609: f64, t5825: f64, t706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60311, t60595, t60620, t60624, t60629, t60670) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2593(t10368, t56, t1518, t670, t1921, t5789, t1913, t5808, t22532, t575, t21661, t602);
        let (t60673, t60823, t61020, t61033, t61037, t61090) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2594(t2246, t5812, t1469, t627, t72, t10605, t18539, t11064, t6075, t37, t5940, t2609, t5825, t706);
    (t60311, t60595, t60620, t60624, t60629, t60670, t60673, t60823, t61020, t61033, t61037, t61090)
}
