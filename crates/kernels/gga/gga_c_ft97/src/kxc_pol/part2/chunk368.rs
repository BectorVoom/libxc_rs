//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 368/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk368<F: Float>(t1943: F, t363: F, t558: F, t72: F, t1526: F, t1527: F, t1942: F, t342: F, t343: F, t564: F, t563: F, t10: F, t144: F, t1542: F) -> (F, F, F, F, F) {
    let t1944 = t1943 * t363;
    let t1948 = t72 * t558;
    let t1952 = t564 - t1942 - t1526 * t1527 * t1944 / F::new(12.0) - t342 * t343 * t1948 / F::new(4.0);
    let t1953 = t1952 * t563;
    let t1956 = t10 * t1542 * t144;
    (t1944, t1948, t1952, t1953, t1956)
}
