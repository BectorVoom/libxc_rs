//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1913/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1913<F: Float>(t15281: F, t4936: F, t1174: F, t3431: F, t4912: F, t1090: F, t7319: F, t4919: F, t11531: F, t11534: F, t11537: F, t11541: F, t11591: F, t15265: F, t15269: F, t15274: F, t15278: F, t3447: F) -> (F, F) {
    let t15282 = t15281 * t4936;
    let t15284 = F::cast_from(0.55555555555555555554e-3_f64) * t1174 * t15282;
    let t15285 = t3431 * t4912;
    let t15287 = F::cast_from(0.18518518518518518518e-3_f64) * t1174 * t15285;
    let t15288 = t7319 * t1090;
    let t15289 = t4919 * t15288;
    let t15292 = F::cast_from(0.12345679012345679012e-3_f64) * t11531 - F::cast_from(0.9259259259259259259e-4_f64) * t11534 - F::cast_from(0.18518518518518518518e-3_f64) * t11537 + F::cast_from(0.12345679012345679012e-3_f64) * t11541 + F::cast_from(0.18518518518518518518e-3_f64) * t11591 + F::cast_from(0.49382716049382716049e-3_f64) * t15265 - F::cast_from(0.16666666666666666666e-2_f64) * t1174 * t15269 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t15274 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t15278 - t15284 - t15287 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t15289;
    (t15288, t15292)
}
