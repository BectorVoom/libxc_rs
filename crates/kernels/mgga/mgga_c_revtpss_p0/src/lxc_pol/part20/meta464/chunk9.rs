//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1773/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1773<F: Float>(t4066: F, t4086: F, t786: F, t4104: F, t2782: F, t4100: F, t46433: F, t10022: F, t2453: F, t281: F, t4003: F, t46507: F) -> (F, F, F) {
    let t47423 = t786 * t4086 * t4066;
    let t47424 = t47423 * t4104;
    let t47427 = t2782 * t4100 * t46433;
    let t47429 = t2453 * t10022;
    let t47432 = t47429 * t281 * t46507 * t4003;
    (t47424, t47427, t47432)
}
