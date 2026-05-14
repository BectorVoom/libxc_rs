//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 356/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk356<F: Float>(t1952: F, t563: F, t10: F, t144: F, t1542: F, t1546: F, t520: F, t89: F) -> (F, F, F, F) {
    let t1953 = t1952 * t563;
    let t1956 = t10 * t1542 * t144;
    let t1957 = 2.0 / 27.0 * t1956;
    let t1959 = t89 * t1546 * t520;
    (t1953, t1956, t1957, t1959)
}
