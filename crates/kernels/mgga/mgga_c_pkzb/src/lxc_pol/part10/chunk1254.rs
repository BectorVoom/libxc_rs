//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1254/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1254<F: Float>(t19822: F, t19824: F, t20325: F, t16876: F, t20332: F, t20334: F, t124: F, t24596: F, t20336: F, t1667: F, t8717: F, t20340: F, t16897: F, t16701: F, t16721: F, t16873: F, t16875: F, t16886: F, t16889: F, t16893: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24633 = 0.69263436422725855034e2 * t19822;
    let t24634 = 8.0 * t19824;
    let t24635 = 2.0 * t20325;
    let t24636 = 8.0 * t16876;
    let t24637 = 8.0 * t20332;
    let t24638 = 24.0 * t20334;
    let t24640 = 0.19751673498613801407e-1 * t24596 * t124;
    let t24641 = 24.0 * t20336;
    let t24642 = t8717 * t1667;
    let t24643 = 0.24415263074675393405e-3 * t24642;
    let t24644 = 0.11696447245269292414e1 * t20340;
    let t24645 = 2.0 * t16897;
    let t24646 = t16873 + t16701 - t24633 + t24634 + t24635 - t16875 - t24636 - t24637 - t24638 + t24640 - t24641 + t24643 - t16886 - t16889 - t24644 + t16893 + t24645 + t16721;
    (t24633, t24634, t24635, t24636, t24637, t24638, t24640, t24641, t24643, t24644, t24645, t24646)
}
