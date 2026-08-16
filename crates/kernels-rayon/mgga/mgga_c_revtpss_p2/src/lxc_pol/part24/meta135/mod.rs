//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk714;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk715;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk716;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta135(t1450: f64, t1907: f64, t198: f64, t530: f64, t532: f64, t1317: f64, t1857: f64, t1320: f64, t1468: f64, t3833: f64, t1711: f64, t3841: f64, t1856: f64, t749: f64, t512: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5532 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk714(t1450, t1907);
        let t5536 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk715(t198, t530);
        let (t5541, t5545, t5547, t5549, t5557, t5569) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk716(t198, t532, t1317, t1857, t1320, t1468, t3833, t1711, t3841, t1856, t749);
        let (t5570, t5571) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk717(t512, t5569, t177, t1856);
    (t5532, t5536, t5541, t5545, t5547, t5549, t5557, t5569, t5570, t5571)
}
