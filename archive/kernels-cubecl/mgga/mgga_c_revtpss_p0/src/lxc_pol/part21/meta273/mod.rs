//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta273 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1493;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1494;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1495;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1496;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta273<F: Float>(t4012: F, t828: F, t9984: F, t1384: F, t235: F, t239: F, t820: F, t4003: F, t543: F, t9898: F, t1390: F, t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F, t1410: F, t3934: F, t3944: F, t9932: F, t9937: F, t9944: F, t9953: F, t9958: F, t9963: F, t9966: F, t9971: F, t9973: F, t9977: F, t9982: F, t9755: F, t9824: F, t9928: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9986, t9989, t9990, t9991) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1493::<F>(t4012, t828, t9984, t1384, t235);
        let (t9993, t9994) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1494::<F>(t239, t820, t9991, t4003, t543);
        let t9995 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1495::<F>(t9898, t9994);
        let (t9997, t10001, t10003, t10004, t10006) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1496::<F>(t1390, t828, t9995, t2482, t27, t4000, t221, t4004, t4019, t1410, t3934, t3944, t9932, t9937, t9944, t9953, t9958, t9963, t9966, t9971, t9973, t9977, t9982, t9986, t9993);
        let t10008 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1497::<F>(t10006, t9755, t9824, t9928);
    (t9986, t9989, t9990, t9991, t9994, t9995, t9997, t10001, t10003, t10004, t10008)
}
