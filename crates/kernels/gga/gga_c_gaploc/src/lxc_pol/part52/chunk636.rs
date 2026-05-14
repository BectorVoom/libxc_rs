//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 636/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk636<F: Float>(t13847: F, t969: F, t825: F, t2610: F, t3720: F, t2365: F, t2033: F, t12252: F, t959: F, t12693: F, t12706: F, t12223: F, t2562: F, t883: F, t943: F, t2558: F, t3732: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13891 = t2610 * t3720;
    let t13892 = t2365 * t13891;
    let t13893 = t2033 * t13892;
    let t13895 = t12252 * t959;
    let t13898 = 0.63904876589867916128e-1 * t12693;
    let t13899 = 0.63904876589867916128e-1 * t12706;
    let t13934 = t2562 * t883 * t12223;
    let t13935 = t943 * t13934;
    let t13937 = t3732 * t2558;
    (t13851, t13852, t13891, t13892, t13893, t13895, t13898, t13899, t13934, t13935, t13937)
}
