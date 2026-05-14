//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1238/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1238<F: Float>(t1753: F, t8914: F, t5257: F, t8964: F, t2575: F, t6758: F, t1769: F, t8988: F, t3457: F, t5296: F, t3401: F, t568: F, t1020: F, t1634: F, t16373: F, t16467: F, t17034: F, t1706: F, t1733: F, t1734: F, t1774: F, t179: F, t24091: F, t3402: F, t3406: F, t5225: F, t5279: F, t581: F, t612: F, t616: F, t6970: F, t6979: F, t9003: F, t9011: F) -> (F, F, F, F, F) {
    let t24151 = t8914 * t1753;
    let t24155 = t5257 * t8964;
    let t24161 = t6758 * t2575;
    let t24169 = t1769 * t8988;
    let t24171 = t5296 * t3457;
    let t24189 = t3401 * t568;
    let t24194 = t1020 * t2575;
    let t24204 = 0.85748036236139473944e-2 * t612 * t1774 * t616 * t24091 + 0.80031500487063509014e-2 * t24169 - 0.22675591804667994221e-1 * t24171 + 5.0 / 4.0 * t16373 * t581 * t3402 * t1634 - t5225 * t581 * t3406 * t1634 / 4.0 - 0.17149607247227894789e-1 * t5279 * t179 * t6970 * t6758 + 0.34299214494455789578e-2 * t1733 * t179 * t6979 * t9003 + 0.51448821741683684366e-1 * t17034 * t179 * t24189 * t1734 - 0.17149607247227894789e-1 * t5279 * t179 * t24194 * t1734 - 0.56688979511669985553e-2 * t16467 + t1706 * t581 * t9011 * t568 / 8.0;
    (t24151, t24155, t24161, t24194, t24204)
}
