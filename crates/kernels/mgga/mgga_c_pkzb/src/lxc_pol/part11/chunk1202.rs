//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1202/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1202<F: Float>(t2639: F, t8953: F, t10572: F, t1734: F, t10556: F, t164: F, t17088: F, t17096: F, t1733: F, t179: F, t24461: F, t24487: F, t24489: F, t24729: F, t2575: F, t2593: F, t2645: F, t2646: F, t2653: F, t29067: F, t3396: F, t50: F, t5244: F, t5279: F, t580: F, t581: F, t600: F, t6758: F, t6896: F, t6970: F, t8904: F, t8909: F, t8948: F, t8981: F, t9003: F) -> (F, F, F) {
    let t29424 = t8953 * t2639;
    let t29454 = t10572 * t1734;
    let t29475 = -F::cast_from(0.38586616306262763276e-2_f64) * t6896 * t179 * t29424 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t179 * t10556 * t600 * t164 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t8904 * t2575 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t6970 * t3396 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t2646 * t164 * t3396 - F::cast_from(0.12862205435420921092e-1_f64) * t5279 * t179 * t8904 * t6758 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t8909 * t9003 - F::cast_from(0.64311027177104605458e-3_f64) * t2645 * t179 * t29454 - F::cast_from(0.24009450146119052704e-1_f64) * t24461 + t17088 + F::cast_from(0.15117061203111996148e0_f64) * t17096 - F::cast_from(0.51448821741683684366e-2_f64) * t5244 * t179 * t8948 * t2653 - F::cast_from(0.51448821741683684366e-2_f64) * t5244 * t179 * t2593 * t8981 + F::cast_from(0.34013387707001991332e-1_f64) * t24487 - F::cast_from(0.17006693853500995666e-1_f64) * t24489 - t580 * t581 * t50 * t29067 / F::new(48.0) - F::cast_from(0.6002362536529763176e-1_f64) * t24729;
    (t29424, t29454, t29475)
}
