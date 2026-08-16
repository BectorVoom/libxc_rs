//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1887;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1888;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta414(t1466: f64, t2246: f64, t1497: f64, t2248: f64, t4241: f64, t644: f64, t2315: f64, t10355: f64, t1469: f64, t2251: f64, t2275: f64, t4186: f64, t30: f64, t33: f64, t606: f64, t2258: f64, t4201: f64, t580: f64, t9342: f64, zeta_threshold: f64, t48: f64, t10368: f64, t2282: f64, t4210: f64, t60: f64, t10379: f64, t1474: f64, t1480: f64, t2270: f64, t2283: f64, t2286: f64, t4202: f64, t4205: f64, t44: f64, t56: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13272, t13283, t13286, t13289, t13299, t13302) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1887(t1466, t2246, t1497, t2248, t4241, t644, t2315, t10355, t1469, t2251, t2275, t4186);
        let (t13303, t13306, t13312) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1888(t30, t33, t13302, t606, t2258, t4201, t580, t9342, zeta_threshold);
        let (t13313, t13324, t13334) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1889(t13312, t48, t10368, t1469, t2251, t2282, t4186, t606, t2258, t4210, t60, t10379, t13299, t13303, t13306, t1474, t1480, t2270, t2283, t2286, t4202, t4205, t44, t56, t614);
    (t13272, t13283, t13286, t13289, t13299, t13302, t13303, t13306, t13312, t13313, t13324, t13334)
}
