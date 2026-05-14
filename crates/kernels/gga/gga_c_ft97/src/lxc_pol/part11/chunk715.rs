//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 715/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk715<F: Float>(t1882: F, t2864: F, t2894: F, t684: F, t835: F, t2850: F, t319: F, t9587: F, t2889: F, t2857: F, t9578: F, t10394: F, t10400: F, t10276: F, t848: F, t9938: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10533 = t1882 * t2864;
    let t10536 = t835 * t2894 * t684;
    let t10539 = t1882 * t2850;
    let t10542 = t835 * t319 * t9587;
    let t10545 = t1882 * t2889;
    let t10548 = t2857 * t319 * t9578;
    let t10552 = t10394 / 3.0;
    let t10553 = 4.0 / 9.0 * t10400;
    let t10555 = 2.0 / 3.0 * t10276;
    let t10556 = t848 * t9938;
    (t10533, t10536, t10539, t10542, t10545, t10548, t10552, t10553, t10555, t10556)
}
