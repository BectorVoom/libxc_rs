//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2604;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2605;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta740<F: Float>(t13726: F, t9303: F, t13725: F, t1445: F, t2439: F, t14082: F, t3920: F, t14078: F, t2470: F, t3915: F, t13735: F, t2435: F, t10119: F, t14114: F, t10115: F, t1900: F, t14189: F, t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F, t1892: F, t3923: F, t2782: F, t4003: F, t5744: F, t10069: F, t14124: F, t14129: F, t14231: F, t14255: F, t4057: F, t46443: F, t46448: F, t46452: F, t46454: F, t46458: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47938, t47942, t47945, t47948, t47952) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2604::<F>(t13726, t9303, t13725, t1445, t2439, t14082, t3920, t14078, t2470, t3915, t13735, t2435);
        let (t47953, t47957, t47961, t47964, t47967, t47971) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2605::<F>(t47952, t10119, t14114, t10115, t1900, t14189, t2435, t22, t46389, t543, t5735, t1432, t5763, t9288);
        let (t47973, t47992) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2606::<F>(t1892, t3923, t2782, t4003, t5744, t10069, t14124, t14129, t14231, t14255, t4057, t46443, t46448, t46452, t46454, t46458, t47971, t820);
    (t47938, t47942, t47945, t47948, t47953, t47957, t47961, t47964, t47967, t47973, t47992)
}
