//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2088;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta445(t1580: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t10503: f64, t10507: f64, t10511: f64, t10984: f64, t10987: f64, t14998: f64, t15004: f64, t15006: f64, t15010: f64, t15011: f64, t2829: f64, t4474: f64, t887: f64, t4533: f64, t886: f64, t2770: f64, t1579: f64, t2828: f64, t10989: f64, t10992: f64, t10998: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11022: f64, t2765: f64, t4487: f64, t4534: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15014, t15015, t15017, t15018, t15022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2088(t1580, t2440, t2439, t1569, t2453, t2458, t10503, t10507, t10511, t10984, t10987, t14998, t15004, t15006, t15010, t15011, t2829, t4474, t887);
        let (t15029, t15030, t15038, t15044) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2089(t4533, t886, t2770, t1579, t2828, t10989, t10992, t10998, t11000, t11004, t11013, t11017, t11019, t11022, t2765, t4487, t4534, t865);
    (t15014, t15015, t15017, t15018, t15022, t15029, t15030, t15038, t15044)
}
