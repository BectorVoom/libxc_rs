//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2095;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta639(t28019: f64, t531: f64, t1513: f64, t94975: f64, t28036: f64, t94978: f64, t25823: f64, t4287: f64, t1913: f64, t7337: f64, t116: f64, t28042: f64, t28283: f64, t571: f64, t28234: f64, t575: f64, t1455: f64, t7956: f64, t1464: f64, t7939: f64, t2037: f64, t5808: f64, t1921: f64, t7318: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101417, t101451, t101454, t101456, t101563, t101622) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2095(t28019, t531, t1513, t94975, t28036, t94978, t25823, t4287, t1913, t7337, t116, t28042);
        let (t101656, t101658, t101661, t101668, t101670, t101672) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2096(t28283, t571, t28234, t575, t1455, t7956, t1464, t7939, t2037, t5808, t1921, t7318);
    (t101417, t101451, t101454, t101456, t101563, t101622, t101656, t101658, t101661, t101668, t101670, t101672)
}
