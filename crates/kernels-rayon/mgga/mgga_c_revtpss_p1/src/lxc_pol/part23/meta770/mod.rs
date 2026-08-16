//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta770 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2571;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta770(t247: f64, t44545: f64, t5230: f64, t5384: f64, t12984: f64, t5327: f64, t17303: f64, t3667: f64, t12627: f64, t489: f64, t17728: f64, t13011: f64, t5373: f64, t1222: f64, t5368: f64, t697: f64, t3625: f64, t44250: f64, t5406: f64, t3781: f64, t5219: f64, t5330: f64, t12881: f64, t5391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57242, t57251, t57257, t57264, t57265, t57270) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2571(t247, t44545, t5230, t5384, t12984, t5327, t17303, t3667, t12627, t489, t17728, t13011, t5373);
        let (t57271, t57274, t57331, t57382, t57421) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2572(t57270, t1222, t5368, t697, t3625, t44250, t5406, t3781, t5219, t5330, t12881, t5391);
    (t57242, t57251, t57257, t57264, t57265, t57271, t57274, t57331, t57382, t57421)
}
