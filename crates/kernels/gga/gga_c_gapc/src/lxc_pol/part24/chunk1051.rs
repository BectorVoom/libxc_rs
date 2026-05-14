//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1051/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1051<F: Float>(t11588: F, t27043: F, t35175: F, t3703: F, t11418: F, t3141: F, t34863: F, t505: F, t128: F, t567: F, t5741: F, t681: F, t11569: F, t659: F, t11558: F, t11408: F, t27889: F, t563: F) -> (F, F, F, F, F, F, F) {
    let t35287 = t11588 * t27043;
    let t35289 = t35175 * t3703;
    let t35293 = t11418 * t3141 * t34863 * t505;
    let t35298 = t11418 * t5741 * t681 * t128 * t567;
    let t35302 = t11569 * t659;
    let t35304 = t35175 * t11558;
    let t35307 = t563 * t11408 * t27889;
    (t35287, t35289, t35293, t35298, t35302, t35304, t35307)
}
