//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2556;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta718<F: Float>(t1399: F, t3960: F, t9816: F, t9818: F, t2735: F, t5744: F, t808: F, t9935: F, t9845: F, t9930: F, t9769: F, t2713: F, t3964: F, t9703: F, t4086: F, t9801: F, t9846: F, t9744: F, t9966: F, t3855: F, t3860: F, t1320: F, t9545: F, t3857: F, t40082: F, t512: F, t520: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46922, t46931, t46934, t46941, t46944) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2556::<F>(t1399, t3960, t9816, t9818, t2735, t5744, t808, t9935, t9845, t9930, t9769, t2713, t3964, t9703);
        let (t46946, t46947, t46949, t46960, t46963, t46967, t46970) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2557::<F>(t4086, t9801, t9846, t9744, t9966, t3855, t3860, t1320, t9545, t3857, t40082, t512, t520);
    (t46922, t46931, t46934, t46941, t46944, t46946, t46947, t46949, t46960, t46963, t46967, t46970)
}
