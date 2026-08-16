//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1797;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1798;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1799;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1800;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta381(t12295: f64, t12351: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t12344: f64, t12347: f64, t12354: f64, t12448: f64, t1169: f64, t1159: f64, t3475: f64, t426: f64, t3478: f64, t434: f64, t12430: f64, t1179: f64, t3488: f64, t1175: f64, t3520: f64, t3519: f64, t444: f64, t439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12459, t12460, t12463) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1797(t12295, t12351, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t12344, t12347, t12354);
        let (t12464, t12465, t12469, t12470) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1798(t12448, t12463, t1169, t1159, t3475, t426);
        let (t12472, t12473, t12476, t12481) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1799(t3478, t434, t12430, t1179, t3488, t1175, t3520);
        let t12485 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1800(t3519, t444);
        let t12486 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1801(t12485, t439);
    (t12459, t12460, t12464, t12465, t12469, t12470, t12472, t12473, t12476, t12481, t12485, t12486)
}
