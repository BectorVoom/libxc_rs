//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1016/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1016<F: Float>(t12567: F, t3271: F, t3016: F, t797: F, t3262: F, t3263: F, t3060: F, t10997: F, t3275: F, t11018: F, t12434: F, t12437: F, t12440: F, t12443: F, t12560: F, t12563: F, t12565: F) -> (F, F, F, F, F, F) {
    let t12568 = t12567 * t3271;
    let t12569 = t12568 / F::new(4.0);
    let t12570 = t797 * t3016;
    let t12572 = t3262 * t3263 * t12570;
    let t12573 = F::new(3.0) / F::new(4.0) * t12572;
    let t12574 = t797 * t3060;
    let t12576 = t3275 * t10997 * t12574;
    let t12577 = F::new(45.0) / F::new(64.0) * t12576;
    let t12578 = t12434 + F::cast_from(0.15243824895787514157e-3_f64) * t12437 - t11018 - F::cast_from(0.36021158228745895953e-3_f64) * t12440 - F::cast_from(0.72042316457491791906e-3_f64) * t12443 - t12560 + t12563 - t12565 - t12569 - t12573 - t12577;
    (t12569, t12570, t12573, t12574, t12577, t12578)
}
