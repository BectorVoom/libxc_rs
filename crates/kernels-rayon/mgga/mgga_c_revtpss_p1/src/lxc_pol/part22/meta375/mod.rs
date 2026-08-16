//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1924;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1925;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta375(t10275: f64, t10278: f64, t10284: f64, t10287: f64, t10295: f64, t13261: f64, t13262: f64, t13263: f64, t13264: f64, t13265: f64, t13266: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64, t1497: f64, t2248: f64, t4241: f64, t644: f64, t2315: f64, t10355: f64, t1469: f64, t2251: f64, t2275: f64, t4186: f64, t30: f64, t33: f64, t606: f64, t2258: f64, t4201: f64, t580: f64, t9342: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13267, t13269) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1924(t10275, t10278, t10284, t10287, t10295, t13261, t13262, t13263, t13264, t13265, t13266, t4171, t602);
        let (t13272, t13283, t13286, t13289, t13299, t13302) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1925(t1466, t2246, t1497, t2248, t4241, t644, t2315, t10355, t1469, t2251, t2275, t4186);
        let (t13303, t13306, t13309, t13310, t13312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1926(t30, t33, t13302, t606, t2258, t4201, t580, t9342, zeta_threshold);
    (t13267, t13269, t13272, t13283, t13286, t13289, t13299, t13303, t13306, t13309, t13310, t13312)
}
