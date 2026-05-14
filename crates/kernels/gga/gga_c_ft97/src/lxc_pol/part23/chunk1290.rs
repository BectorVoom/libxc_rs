//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1290/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1290<F: Float>(t24429: F, t5147: F, t31126: F, t8392: F, t1882: F, t31279: F, t31303: F, t31236: F, t10007: F, t1091: F, t110024: F, t111405: F, t111420: F, t111436: F, t11593: F, t14159: F, t1456: F, t18139: F, t1901: F, t242: F, t2574: F, t2606: F, t28157: F, t28349: F, t28355: F, t31197: F, t3746: F, t446: F, t4934: F, t53923: F, t6194: F, t684: F, t729: F) -> (F, F) {
    let t124971 = t24429 * t5147;
    let t124976 = t8392 * t31126;
    let t124996 = t1882 * t31279;
    let t125002 = t8392 * t31303;
    let t125008 = t1882 * t31236;
    let t125010 = t111405 - t446 * t242 * t124971 / 3.0 - 8.0 / 27.0 * t111420 + 4.0 / 27.0 * t124976 - t1901 * t10007 * t31197 * t684 / 9.0 - 2.0 / 9.0 * t1901 * t53923 * t28349 - 4.0 / 9.0 * t11593 * t14159 * t28157 - 4.0 / 9.0 * t11593 * t2606 * t28355 * t3746 + 2.0 / 3.0 * t446 * t2574 * t6194 * t4934 + t124996 / 9.0 - t111436 + 2.0 / 9.0 * t1901 * t2606 * t110024 * t1091 + 2.0 / 27.0 * t125002 - t446 * t729 * t1456 * t18139 / 3.0 - t125008 / 9.0;
    (t124971, t125010)
}
