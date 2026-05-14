//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 825/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk825<F: Float>(t11343: F, t11344: F, t3669: F, t561: F, t1023: F, t3663: F, t563: F, t2983: F, t3684: F, t659: F, t3707: F, t519: F) -> (F, F, F, F, F, F, F) {
    let t11345 = t11343 * t11344;
    let t11347 = t561 * t3669;
    let t11348 = t11347 * t1023;
    let t11350 = t563 * t3663;
    let t11351 = t11350 * t2983;
    let t11353 = t3684 * t659;
    let t11355 = t3707 * M_PI;
    let t11356 = t519 * t11355;
    (t11345, t11347, t11348, t11350, t11351, t11353, t11356)
}
