//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta800 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2626;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta800(t18615: f64, t231: f64, t243: f64, t2661: f64, t2662: f64, t14923: f64, t18478: f64, t10811: f64, t18334: f64, t18629: f64, t10777: f64, t10779: f64, t14671: f64, t18637: f64, t50412: f64, t6035: f64, t4321: f64, t4534: f64, t689: f64, t10995: f64, t18312: f64, t686: f64, t72: f64, t18804: f64, t2470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62458, t62460, t62475, t62494, t62498) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2626(t18615, t231, t243, t2661, t2662, t14923, t18478, t10811, t18334, t18629, t10777, t10779, t14671, t18637);
        let (t62502, t62516, t62523, t62528) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2627(t10777, t10779, t50412, t6035, t4321, t4534, t689, t10995, t18312, t686, t72, t18804, t2470);
    (t62458, t62460, t62475, t62494, t62498, t62502, t62516, t62523, t62528)
}
