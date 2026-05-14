//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1372/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1372<F: Float>(t26036: F, t6407: F, t8078: F, t20481: F, t551: F, t574: F, t921: F, t22711: F, t24695: F, t2567: F, t2572: F, t26008: F, t26015: F, t26018: F, t26021: F, t26027: F, t26029: F, t360: F, t495: F, t5108: F, t5109: F, t6136: F, t6219: F, t6364: F, t7542: F, t8029: F) -> (F,) {
    let t26037 = 0.17563392970889009434e0 * t26036;
    let t26038 = t6407 * t8078;
    let t26039 = 0.87816964854445047168e-1 * t26038;
    let t26042 = t574 * t551 * t20481 * t921;
    let t26044 = -t26008 + 0.20803732176130244552e1 * t22711 - 0.7801399566048841707e0 * t5108 * t5109 * t7542 * t495 + 0.29272321618148349056e-1 * t26015 - 0.40752780427737692339e0 * t26018 - t26021 + 0.31205598264195366828e1 * t24695 * t360 * t2567 * t6219 + 0.83214928704520978207e1 * t26027 - 0.26004665220162805689e0 * t26029 * t6136 - 0.15602799132097683414e1 * t8029 * t360 * t2572 * t6364 - t26037 + t26039 + 0.19776387377308997907e1 * t26042;
    (t26044,)
}
