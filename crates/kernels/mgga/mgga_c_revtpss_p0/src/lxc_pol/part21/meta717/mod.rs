//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta717 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2554;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta717<F: Float>(t1413: F, t547: F, t807: F, t9628: F, t3952: F, t9784: F, t281: F, t39644: F, t40650: F, t550: F, t2689: F, t9715: F, t40688: F, t46786: F, t9400: F, t9941: F, t9704: F, t1386: F, t2682: F, t820: F, t3940: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46877, t46879, t46885, t46886) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2554::<F>(t1413, t547, t807, t9628, t3952, t9784, t281, t39644, t40650, t550, t2689, t9715);
        let (t46888, t46889, t46893, t46895, t46917, t46918) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2555::<F>(t40688, t547, t46786, t807, t9400, t9941, t2689, t9704, t1386, t2682, t820, t3940);
    (t46877, t46879, t46885, t46886, t46888, t46889, t46893, t46895, t46917, t46918)
}
