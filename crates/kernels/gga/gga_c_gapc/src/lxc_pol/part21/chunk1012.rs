//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1012/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1012<F: Float>(t19509: F, t2981: F, t19508: F, t1649: F, t5391: F, t137: F, t1552: F, t442: F, t5964: F, t4048: F, t563: F, t1577: F, t19422: F) -> (F, F, F, F, F, F) {
    let t19510 = t19509 * t2981;
    let t19511 = t19508 * t19510;
    let t19522 = t1649 * t5391;
    let t19530 = t5964 * t1552 * t137 * t442;
    let t19533 = t563 * t4048;
    let t19535 = t19422 * t1577;
    (t19510, t19511, t19522, t19530, t19533, t19535)
}
