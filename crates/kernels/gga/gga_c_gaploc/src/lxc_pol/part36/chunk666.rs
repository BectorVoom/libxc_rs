//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 666/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk666<F: Float>(t28023: F, t739: F, t10032: F, t1980: F, t22537: F, t822: F, t2012: F, t9804: F, t1858: F, t3234: F, t325: F, t9688: F, t701: F, t2610: F, t22542: F, t2021: F, t6109: F, t899: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28024 = t739 * t28023;
    let t28028 = t1980 * t10032;
    let t28069 = t822 * t22537;
    let t28073 = t2012 * t9804;
    let t28152 = t1858 * t3234;
    let t28197 = t325 * t9688;
    let t28236 = t3234 * t701;
    let t28302 = t2610 * t28236;
    let t28309 = t822 * t22542;
    let t28412 = t2021 * t6109 * t899;
    (t28024, t28028, t28069, t28073, t28152, t28197, t28236, t28302, t28309, t28412)
}
