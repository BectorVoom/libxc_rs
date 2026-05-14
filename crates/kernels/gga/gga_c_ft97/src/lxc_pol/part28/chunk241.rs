//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 241/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk241<F: Float>(t358: F, t487: F, t342: F, t511: F, t630: F, t142: F, t363: F, t558: F, t72: F, t1526: F, t1527: F, t343: F, t564: F, t10: F, t144: F, t1542: F) -> (F, F, F, F, F, F, F) {
    let t1910 = t487 * t358;
    let t1942 = t342 * t630 * t511 / 12.0;
    let t1943 = t142 * t358;
    let t1944 = t1943 * t363;
    let t1948 = t72 * t558;
    let t1952 = t564 - t1942 - t1526 * t1527 * t1944 / 12.0 - t342 * t343 * t1948 / 4.0;
    let t1956 = t10 * t1542 * t144;
    (t1910, t1942, t1943, t1944, t1948, t1952, t1956)
}
