//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1394/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1394<F: Float>(t410: F, t7733: F, t4889: F, t959: F, t5397: F, t5398: F, t956: F, t584: F, t5861: F, t21202: F, t21206: F, t21210: F, t21216: F, t21221: F, t21225: F, t21228: F, t21232: F, t26420: F) -> (F,) {
    let t26422 = t410 * t7733;
    let t26424 = t4889 * t959;
    let t26427 = t5397 * t956 * t5398;
    let t26430 = t584 * t956 * t5861;
    let t26434 = -0.1016176784e-1 * t21202 + 24.0 * t21206 - 12.0 * t26420 + 12.0 * t26422 + 120.0 * t26424 - 0.6858336e0 * t26427 - 0.571528e-1 * t26430 - t21210 - t21216 - t21221 - 0.36464057928e1 * t21225 + 0.92286169723947659921e4 * t21228 + t21232;
    (t26434,)
}
