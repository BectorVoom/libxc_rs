//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1797;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1798;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1799;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1800;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta381<F: Float>(t12295: F, t12351: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t12344: F, t12347: F, t12354: F, t12448: F, t1169: F, t1159: F, t3475: F, t426: F, t3478: F, t434: F, t12430: F, t1179: F, t3488: F, t1175: F, t3520: F, t3519: F, t444: F, t439: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12459, t12460, t12463) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1797::<F>(t12295, t12351, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t12344, t12347, t12354);
        let (t12464, t12465, t12469, t12470) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1798::<F>(t12448, t12463, t1169, t1159, t3475, t426);
        let (t12472, t12473, t12476, t12481) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1799::<F>(t3478, t434, t12430, t1179, t3488, t1175, t3520);
        let t12485 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1800::<F>(t3519, t444);
        let t12486 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1801::<F>(t12485, t439);
    (t12459, t12460, t12464, t12465, t12469, t12470, t12472, t12473, t12476, t12481, t12485, t12486)
}
