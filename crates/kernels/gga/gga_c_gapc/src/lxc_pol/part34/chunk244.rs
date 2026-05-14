//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 244/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk244<F: Float>(t332: F, t818: F, t918: F, t197: F, t277: F, t668: F, t296: F, t328: F) -> (F, F, F, F, F) {
    let t952 = t332 * t818;
    let t953 = t918 * t952;
    let t954 = t197 * t953;
    let t957 = t277 * t668;
    let t959 = 1.0 / t328 / t296;
    (t952, t953, t954, t957, t959)
}
