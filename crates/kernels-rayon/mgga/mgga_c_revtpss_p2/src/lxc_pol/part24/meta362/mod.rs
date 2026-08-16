//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1234;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1235;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta362(t1733: f64, t6470: f64, t3384: f64, t1732: f64, t20644: f64, t3433: f64, t17092: f64, t6439: f64, t6438: f64, t1150: f64, t12256: f64, t22688: f64, t12305: f64, t128: f64, t12268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24212, t24214, t24215, t24217, t24219, t24220, t24221, t24223, t24228) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1234(t1733, t6470, t3384, t1732, t20644, t3433, t17092, t6439, t6438, t1150, t12256, t22688);
        let (t24229, t24230) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1235(t12305, t24228, t128);
        let t24232 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1236(t12268, t22688);
    (t24212, t24214, t24215, t24217, t24219, t24220, t24221, t24223, t24228, t24229, t24230, t24232)
}
