//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1361/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1361<F: Float>(t26282: F, t9247: F, t113: F, t32532: F, t2155: F, t6063: F, t20328: F, t2184: F, t24901: F, t24905: F, t24908: F, t24911: F, t24915: F, t24918: F, t2719: F, t29443: F, t29447: F, t3216: F, t551: F, t552: F, t7383: F, t9117: F, t9365: F, t938: F) -> (F, F) {
    let t33286 = t26282 * t9247;
    let t33288 = t32532 * t113;
    let t33290 = t2155 * t6063 * t33288;
    let t33309 = 0.1047928639570397803e0 * t33286 + 0.29272321618148349055e-1 * t33290 + 0.41530324072742201648e-1 * t20328 - t24901 - t24905 - 0.12459097221822660494e0 * t24908 - t24911 - t24915 - 0.37377291665467981483e0 * t24918 - 0.12713391885412927226e1 * t29443 + 0.76280351312477563357e1 * t29447 + 0.15602799132097683414e1 * t7383 * t9117 + 0.26004665220162805689e0 * t2184 * t551 * t552 * t2719 * t3216 + 0.26004665220162805689e0 * t2184 * t551 * t552 * t938 * t9365;
    (t33288, t33309)
}
