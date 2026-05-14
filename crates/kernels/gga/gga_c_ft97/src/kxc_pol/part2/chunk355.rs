//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 355/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk355<F: Float>(t86: F, t112: F, t113: F, t1927: F, t1934: F, t5: F, t502: F, t505: F, t342: F, t511: F, t630: F, t142: F, t358: F, t363: F, t558: F, t72: F, t1526: F, t1527: F, t343: F, t564: F) -> (F, F, F, F, F) {
    let t87 = 10000000.0 <= t86;
    let t1939 = piecewise3(t87, 0.0, t5 * t1927 * t113 / 4.0 + t5 * t502 * t505 / 2.0 + t5 * t112 * t1934 / 4.0);
    let t1942 = t342 * t630 * t511 / 12.0;
    let t1943 = t142 * t358;
    let t1944 = t1943 * t363;
    let t1948 = t72 * t558;
    let t1952 = t564 - t1942 - t1526 * t1527 * t1944 / 12.0 - t342 * t343 * t1948 / 4.0;
    (t1939, t1943, t1944, t1948, t1952)
}
