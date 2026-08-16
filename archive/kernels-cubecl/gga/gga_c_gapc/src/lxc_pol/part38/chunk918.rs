//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 918/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk918<F: Float>(t1: F, t11341: F, t169: F, t2974: F, t8676: F, t3669: F, t561: F, t1023: F, t3663: F, t563: F, t2983: F, t3684: F, t659: F) -> (F, F, F, F, F, F, F, F) {
    let t11342 = t11341 * t1;
    let t11343 = t169 * t11342;
    let t11344 = t8676 * t2974;
    let t11345 = t11343 * t11344;
    let t11347 = t561 * t3669;
    let t11348 = t11347 * t1023;
    let t11350 = t563 * t3663;
    let t11351 = t11350 * t2983;
    let t11353 = t3684 * t659;
    (t11342, t11344, t11345, t11347, t11348, t11350, t11351, t11353)
}
