//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1710/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1710<F: Float>(t10022: F, t2782: F, t46422: F, t10013: F, t2453: F, t10142: F, t136: F, t2457: F, t3964: F, t4066: F, t10139: F, t1398: F, t281: F, t543: F, t624: F) -> (F, F, F, F) {
    let t46493 = t2782 * t10022 * t46422;
    let t46495 = t2453 * t10013;
    let t46496 = t46495 * t10142;
    let t46500 = t3964 * t4066 * t136 * t2457;
    let t46505 = t10139 * t281 * t624 * t1398 * t543;
    (t46493, t46496, t46500, t46505)
}
