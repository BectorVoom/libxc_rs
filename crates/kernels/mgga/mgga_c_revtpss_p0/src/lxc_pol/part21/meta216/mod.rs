//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1295;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1296;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1297;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1298;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1299;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1300;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta216<F: Float>(t1187: F, t3523: F, t5205: F, t1196: F, t3358: F, t3546: F, t5044: F, t5049: F, t5054: F, t5058: F, t459: F, t1208: F, t1769: F, t487: F, t1770: F, t1214: F, t1774: F, t1211: F, t1294: F, t1277: F, t3579: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5206, t5207, t5209, t5215, t5216) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1294::<F>(t1187, t3523, t5205, t1196, t3358, t3546, t5044, t5049, t5054, t5058, t459);
        let t5219 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1295::<F>(t1208, t1769);
        let t5220 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1296::<F>(t487, t5219);
        let t5225 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1297::<F>(t1770, t487);
        let t5230 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1298::<F>(t1214, t1774);
        let t5231 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1299::<F>(t1211, t5230);
        let t5237 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1300::<F>(t1294, t1774, t1277);
        let t5245 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1301::<F>(t3358, t3579, t5044, t5049, t5054, t5058);
    (t5206, t5207, t5209, t5215, t5216, t5219, t5220, t5225, t5230, t5231, t5237, t5245)
}
