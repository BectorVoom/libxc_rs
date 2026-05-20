//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1736/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1736<F: Float>(t46929: F, t808: F, t9935: F, t9845: F, t9930: F, t9769: F, t2713: F, t3964: F, t9703: F, t4086: F, t9801: F, t9846: F) -> (F, F, F, F, F) {
    let t46931 = t46929 * t808 * t9935;
    let t46934 = t9845 * t808 * t9930;
    let t46941 = t9845 * t808 * t9769;
    let t46944 = t3964 * t2713 * t9703;
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    (t46931, t46934, t46941, t46944, t46947)
}
