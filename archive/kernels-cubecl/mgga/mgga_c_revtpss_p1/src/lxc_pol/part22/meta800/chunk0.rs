//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2901/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2901<F: Float>(t3940: F, t46917: F, t3829: F, t4003: F, t2735: F, t5744: F, t808: F, t9935: F, t9845: F, t9930: F, t9769: F, t2713: F, t3964: F, t9703: F) -> (F, F, F, F, F, F, F) {
    let t46918 = t46917 * t3940;
    let t46924 = t4003 * t3829;
    let t46929 = t2735 * t5744;
    let t46931 = t46929 * t808 * t9935;
    let t46934 = t9845 * t808 * t9930;
    let t46941 = t9845 * t808 * t9769;
    let t46944 = t3964 * t2713 * t9703;
    (t46918, t46924, t46929, t46931, t46934, t46941, t46944)
}
