//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta798 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2622;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta798(t14370: f64, t18259: f64, t18562: f64, t2626: f64, t14330: f64, t5819: f64, t606: f64, t749: f64, t162: f64, t50089: f64, t2609: f64, t5944: f64, t18263: f64, t2615: f64, t2475: f64, t5962: f64, t10696: f64, t5966: f64, t18616: f64, t221: f64, t2484: f64, t2485: f64, t10815: f64, t5980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62274, t62276, t62282, t62291, t62300) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2622(t14370, t18259, t18562, t2626, t14330, t5819, t606, t749, t162, t50089, t2609, t5944);
        let (t62302, t62351, t62361, t62392, t62399) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2623(t18263, t2615, t2475, t5962, t10696, t5966, t18616, t221, t2484, t2485, t10815, t5980);
    (t62274, t62276, t62282, t62291, t62300, t62302, t62351, t62361, t62392, t62399)
}
