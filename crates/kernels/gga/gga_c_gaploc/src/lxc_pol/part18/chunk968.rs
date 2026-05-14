//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 968/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk968<F: Float>(t21488: F, t795: F, t805: F, t2571: F, t5397: F, t2101: F, t165: F, t2089: F, t1865: F, t935: F, t16788: F, t278: F, t481: F, t16922: F, t16889: F, t2547: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21490 = t21488 * t805 * t795;
    let t21491 = t2571 * t5397;
    let t21497 = t21488 * t805 * t2101;
    let t21502 = t165 * t2089;
    let t21503 = t935 * t1865;
    let t21504 = t21502 * t21503;
    let t21556 = t481 * t16788 * t278;
    let t21571 = t481 * t16922 * t278;
    let t21636 = t481 * t16889 * t278;
    let t21665 = t481 * t2547 * t686;
    (t21490, t21491, t21497, t21502, t21503, t21504, t21556, t21571, t21636, t21665)
}
