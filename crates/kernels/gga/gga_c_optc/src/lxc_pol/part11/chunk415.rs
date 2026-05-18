//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 415/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk415<F: Float>(t183: F, t2211: F, t2213: F, t2218: F, t2219: F, t2224: F, t724: F, t727: F, t102: F, t108: F, t176: F) -> (F, F) {
    let t2226 = t2211 * t183 - F::new(2.0) * t2213 * t727 + F::new(2.0) * t2218 * t2219 - t724 * t2224;
    let t2227 = t2226 * t102;
    let t2229 = t176 * t2227 * t108;
    (t2226, t2229)
}
