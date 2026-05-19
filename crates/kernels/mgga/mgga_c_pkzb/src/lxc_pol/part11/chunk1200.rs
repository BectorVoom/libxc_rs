//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1200/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1200<F: Float>(t10627: F, t164: F, t2639: F, t8962: F, t600: F, t3441: F, t6864: F, t2593: F, t8888: F, t1020: F, t154: F, t16343: F, t17026: F, t1733: F, t179: F, t19932: F, t20164: F, t20202: F, t20205: F, t24300: F, t24347: F, t24370: F, t24381: F, t24387: F, t2592: F, t2645: F, t29094: F, t3401: F, t5279: F, t568: F, t6970: F, t8976: F) -> (F, F, F, F, F) {
    let t29356 = t10627 * t164;
    let t29361 = t8962 * t2639;
    let t29366 = t10627 * t600 * t164;
    let t29370 = t6864 * t3441;
    let t29374 = t2593 * t8888;
    let t29384 = -F::cast_from(0.68026775414003982663e-1_f64) * t20164 - F::new(3.0) / F::new(4.0) * t19932 * t154 * t29094 + F::cast_from(0.13605355082800796532e0_f64) * t24347 - F::cast_from(0.24009450146119052704e-1_f64) * t24370 - F::cast_from(0.12862205435420921092e-1_f64) * t5279 * t179 * t6970 * t3401 + t17026 + F::cast_from(0.25724410870841842184e-1_f64) * t16343 * t179 * t2593 * t8976 + t20202 + F::cast_from(0.68026775414003982664e-1_f64) * t20205 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t179 * t29356 * t568 - F::cast_from(0.64311027177104605458e-3_f64) * t2645 * t179 * t29361 - F::cast_from(0.21437009059034868486e-3_f64) * t2645 * t179 * t29366 + F::cast_from(0.12862205435420921092e-2_f64) * t2592 * t179 * t29370 + F::cast_from(0.12862205435420921092e-2_f64) * t2592 * t179 * t29374 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t24300 * t1020 + F::cast_from(0.36014175219178579057e0_f64) * t24381 - F::cast_from(0.12004725073059526352e0_f64) * t24387;
    (t29361, t29366, t29370, t29374, t29384)
}
