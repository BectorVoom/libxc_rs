//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 801/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk801<F: Float>(t2876: F, t6334: F, t15299: F, t2883: F, t10703: F, t6386: F, t875: F, t2843: F, t296: F, t1476: F, t2682: F, t10683: F, t319: F, t1483: F, t8232: F, t2894: F, t840: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24936 = t6334 * t2876;
    let t24937 = t15299 * t24936;
    let t24940 = t6334 * t2883;
    let t24941 = t10703 * t24940;
    let t24944 = t6386 * t875;
    let t24945 = t2843 * t24944;
    let t24946 = t296 * t24945;
    let t24949 = t1476 * t2682;
    let t24951 = t10683 * t319 * t24949;
    let t24955 = 4.0 / 27.0 * t8232 * t1483;
    let t24957 = t840 * t2894 * t1476;
    (t24936, t24937, t24940, t24941, t24944, t24945, t24946, t24949, t24951, t24955, t24957)
}
