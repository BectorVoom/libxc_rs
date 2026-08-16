//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2584/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2584<F: Float>(t1174: F, t457: F, t4936: F, t698: F, t15277: F, t3431: F, t15281: F, t15303: F, t11540: F, t4889: F, t11529: F, t4912: F) -> (F, F, F, F, F) {
    let t52354 = t1174 * t698 * t457 * t4936;
    let t52355 = F::cast_from(0.55555555555555555554e-3_f64) * t52354;
    let t52357 = t1174 * t3431 * t15277;
    let t52362 = t1174 * t15281 * t15303;
    let t52364 = t4889 * t11540;
    let t52367 = t1174 * t11529 * t4912;
    (t52355, t52357, t52362, t52364, t52367)
}
