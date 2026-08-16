//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1021/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1021<F: Float>(t73639: F, t920: F, t20045: F, t942: F, t20023: F, t20035: F, t4462: F, t4495: F, t4436: F, t73345: F, t4458: F, t4454: F) -> (F, F, F, F, F, F, F, F, F) {
    let t86054 = t73639 * t920;
    let t86058 = t20045 * t942;
    let t86068 = t20023 * t942;
    let t86075 = t20035 * t942;
    let t86082 = t4462 * t4495;
    let t86086 = t4462 * t4436;
    let t86090 = t73345 * t920;
    let t86094 = t4458 * t4436;
    let t86098 = t4454 * t4436;
    (t86054, t86058, t86068, t86075, t86082, t86086, t86090, t86094, t86098)
}
