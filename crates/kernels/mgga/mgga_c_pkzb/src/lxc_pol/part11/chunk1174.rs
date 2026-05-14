//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1174/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1174<F: Float>(t237: F, t31151: F, t31191: F, t31309: F, t31345: F, t31391: F, t31437: F, t31517: F, t31587: F, t1217: F, t27501: F, t11353: F, t2328: F, t10370: F, t1306: F, t31109: F, t31111: F, t31113: F, t31115: F, t31117: F, t31122: F, t31124: F, t3282: F) -> (F, F, F, F) {
    let t31591 = t237 * (t31151 + t31191 + t31309 + t31345 + t31391 + t31437 + t31517 + t31587);
    let t31593 = 0.17544670867903938621e1 * t27501 * t1217;
    let t31595 = 0.5848223622634646207e0 * t2328 * t11353;
    let t31596 = 6.0 * t10370 * t1306 * t3282 + t31109 - t31111 + t31113 + t31115 - t31117 - t31122 - t31124 + t31591 - t31593 - t31595;
    (t31591, t31593, t31595, t31596)
}
