//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1207/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1207<F: Float>(t1266: F, t3696: F, t3703: F, t11558: F, t34337: F, t11409: F, t27286: F, t11414: F, t26996: F, t11413: F, t27290: F, t563: F) -> (F, F, F, F, F) {
    let t34905 = t1266 * t3696 * t3703;
    let t34907 = t34337 * t11558;
    let t34909 = t11409 * t27286;
    let t34911 = t11414 * t26996;
    let t34914 = t563 * t11413 * t27290;
    (t34905, t34907, t34909, t34911, t34914)
}
