//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1200/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1200(t10627: f64, t164: f64, t2639: f64, t8962: f64, t600: f64, t3441: f64, t6864: f64, t2593: f64, t8888: f64, t1020: f64, t154: f64, t16343: f64, t17026: f64, t1733: f64, t179: f64, t19932: f64, t20164: f64, t20202: f64, t20205: f64, t24300: f64, t24347: f64, t24370: f64, t24381: f64, t24387: f64, t2592: f64, t2645: f64, t29094: f64, t3401: f64, t5279: f64, t568: f64, t6970: f64, t8976: f64) -> (f64, f64, f64, f64, f64) {
    let t29356 = t10627 * t164;
    let t29361 = t8962 * t2639;
    let t29366 = t10627 * t600 * t164;
    let t29370 = t6864 * t3441;
    let t29374 = t2593 * t8888;
    let t29384 = -0.68026775414003982663e-1_f64 * t20164 - 3.0_f64 / 4.0_f64 * t19932 * t154 * t29094 + 0.13605355082800796532e0_f64 * t24347 - 0.24009450146119052704e-1_f64 * t24370 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t6970 * t3401 + t17026 + 0.25724410870841842184e-1_f64 * t16343 * t179 * t2593 * t8976 + t20202 + 0.68026775414003982664e-1_f64 * t20205 + 0.85748036236139473944e-3_f64 * t1733 * t179 * t29356 * t568 - 0.64311027177104605458e-3_f64 * t2645 * t179 * t29361 - 0.21437009059034868486e-3_f64 * t2645 * t179 * t29366 + 0.12862205435420921092e-2_f64 * t2592 * t179 * t29370 + 0.12862205435420921092e-2_f64 * t2592 * t179 * t29374 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t24300 * t1020 + 0.36014175219178579057e0_f64 * t24381 - 0.12004725073059526352e0_f64 * t24387;
    (t29361, t29366, t29370, t29374, t29384)
}
