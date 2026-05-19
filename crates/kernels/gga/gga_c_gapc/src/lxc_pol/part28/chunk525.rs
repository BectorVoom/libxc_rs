//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 525/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk525<F: Float>(t3004: F, t3008: F, t190: F, t671: F, t1649: F, t1643: F, t191: F, t632: F, t1045: F, t198: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3009 = t3004 * t3008;
    let t3011 = t190 * t671;
    let t3012 = t3011 * pi;
    let t3013 = t3012 * t1649;
    let t3014 = t1643 * t3013;
    let t3016 = t632 * t191;
    let t3017 = t1045 * t198;
    (t3009, t3012, t3013, t3014, t3016, t3017)
}
