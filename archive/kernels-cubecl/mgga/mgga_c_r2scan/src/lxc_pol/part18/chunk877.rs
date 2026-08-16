//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 877/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk877<F: Float>(t514: F, t9322: F, t3177: F, t537: F, t255: F, t571: F, t2169: F, t2667: F, t2721: F, t3224: F, t527: F, t566: F, t576: F, t6260: F, t6266: F, t6268: F, t7245: F, t7951: F, t7961: F, t7968: F, t8026: F, t8046: F, t9302: F, t9312: F, t9320: F, t940: F) -> (F, F) {
    let t9323 = t514 * t9322;
    let t9325 = t537 * t3177;
    let t9327 = t571 * t9325 * t255;
    let t9331 = t7951 - t6260 - F::cast_from(0.54878743191129263322e-1_f64) * t527 * t9302 - F::cast_from(0.86682217400542685632e-1_f64) * t7245 * t940 - F::cast_from(0.86682217400542685632e-1_f64) * t2667 * t2721 - F::cast_from(0.13002332610081402845e0_f64) * t2169 * t3224 - F::cast_from(0.13002332610081402845e0_f64) * t566 * t9312 + t6266 - F::cast_from(0.679213007128961539e-1_f64) * t6268 - F::cast_from(0.34930954652346593433e-1_f64) * t7961 + t7968 - F::cast_from(0.54878743191129263322e-2_f64) * t9320 - F::cast_from(0.97574405393827830187e-2_f64) * t9323 - F::cast_from(0.43341108700271342816e-1_f64) * t9327 * t576 - F::cast_from(0.42683466926433871473e0_f64) * t8026 - t8046;
    (t9327, t9331)
}
