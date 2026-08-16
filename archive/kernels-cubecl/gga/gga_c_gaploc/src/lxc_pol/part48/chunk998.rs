//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 998/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk998<F: Float>(t46049: F, t46110: F, t46162: F, t46197: F, t46253: F, t46317: F, t46367: F, t46409: F, t46464: F, t46517: F, t46563: F, t46618: F, t46680: F, t46721: F, t46776: F, t46823: F, t502: F) -> F {
    let t46828 = t502 * (t46049 + t46110 + t46162 + t46197 + t46253 + t46317 + t46367 + t46409 + t46464 + t46517 + t46563 + t46618 + t46680 + t46721 + t46776 + t46823);
    t46828
}
