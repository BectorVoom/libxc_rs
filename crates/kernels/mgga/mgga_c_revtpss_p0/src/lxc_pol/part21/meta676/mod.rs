//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2482;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2483;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2484;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2485;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2486;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta676<F: Float>(t1123: F, t9292: F, t2435: F, t3373: F, t3369: F, t12313: F, t689: F, t12319: F, t2439: F, t3418: F, t12283: F, t698: F, t406: F, t12555: F, t3515: F, t43813: F, t1126: F, t12226: F, t3382: F, t3431: F, t408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t43888 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2482::<F>(t1123, t9292);
        let t43890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2483::<F>(t2435, t3373);
        let t43892 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2484::<F>(t2435, t3369);
        let t43894 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2485::<F>(t12313, t689);
        let t43896 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2486::<F>(t12319, t689);
        let (t43911, t43928, t43946, t43977, t43995, t44012, t44017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2487::<F>(t2439, t3418, t12283, t698, t406, t12555, t3515, t43813, t1126, t12226, t3382, t3431, t408);
    (t43888, t43890, t43892, t43894, t43896, t43911, t43928, t43946, t43977, t43995, t44012, t44017)
}
