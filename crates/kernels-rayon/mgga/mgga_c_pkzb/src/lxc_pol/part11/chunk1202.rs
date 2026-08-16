//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1202/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1202(t2639: f64, t8953: f64, t10572: f64, t1734: f64, t10556: f64, t164: f64, t17088: f64, t17096: f64, t1733: f64, t179: f64, t24461: f64, t24487: f64, t24489: f64, t24729: f64, t2575: f64, t2593: f64, t2645: f64, t2646: f64, t2653: f64, t29067: f64, t3396: f64, t50: f64, t5244: f64, t5279: f64, t580: f64, t581: f64, t600: f64, t6758: f64, t6896: f64, t6970: f64, t8904: f64, t8909: f64, t8948: f64, t8981: f64, t9003: f64) -> (f64, f64, f64) {
    let t29424 = t8953 * t2639;
    let t29454 = t10572 * t1734;
    let t29475 = -0.38586616306262763276e-2_f64 * t6896 * t179 * t29424 + 0.85748036236139473944e-3_f64 * t1733 * t179 * t10556 * t600 * t164 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t8904 * t2575 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t6970 * t3396 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t2646 * t164 * t3396 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t8904 * t6758 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t8909 * t9003 - 0.64311027177104605458e-3_f64 * t2645 * t179 * t29454 - 0.24009450146119052704e-1_f64 * t24461 + t17088 + 0.15117061203111996148e0_f64 * t17096 - 0.51448821741683684366e-2_f64 * t5244 * t179 * t8948 * t2653 - 0.51448821741683684366e-2_f64 * t5244 * t179 * t2593 * t8981 + 0.34013387707001991332e-1_f64 * t24487 - 0.17006693853500995666e-1_f64 * t24489 - t580 * t581 * t50 * t29067 / 48.0_f64 - 0.6002362536529763176e-1_f64 * t24729;
    (t29424, t29454, t29475)
}
