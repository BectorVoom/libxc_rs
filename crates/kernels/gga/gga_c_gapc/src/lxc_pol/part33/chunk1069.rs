//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1069/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1069<F: Float>(t11569: F, t659: F, t11558: F, t35175: F, t11408: F, t27889: F, t563: F, t11405: F, t3085: F, t11342: F, t11344: F, t561: F, t116: F, t1968: F, t204: F, t34159: F) -> (F, F, F, F, F, F) {
    let t35302 = t11569 * t659;
    let t35304 = t35175 * t11558;
    let t35307 = t563 * t11408 * t27889;
    let t35309 = t11405 * t3085;
    let t35312 = t561 * t11342 * t11344;
    let t35316 = t116 * t1968 * t34159 * t204;
    (t35302, t35304, t35307, t35309, t35312, t35316)
}
