//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 877/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk877(t514: f64, t9322: f64, t3177: f64, t537: f64, t255: f64, t571: f64, t2169: f64, t2667: f64, t2721: f64, t3224: f64, t527: f64, t566: f64, t576: f64, t6260: f64, t6266: f64, t6268: f64, t7245: f64, t7951: f64, t7961: f64, t7968: f64, t8026: f64, t8046: f64, t9302: f64, t9312: f64, t9320: f64, t940: f64) -> (f64, f64) {
    let t9323 = t514 * t9322;
    let t9325 = t537 * t3177;
    let t9327 = t571 * t9325 * t255;
    let t9331 = t7951 - t6260 - 0.54878743191129263322e-1_f64 * t527 * t9302 - 0.86682217400542685632e-1_f64 * t7245 * t940 - 0.86682217400542685632e-1_f64 * t2667 * t2721 - 0.13002332610081402845e0_f64 * t2169 * t3224 - 0.13002332610081402845e0_f64 * t566 * t9312 + t6266 - 0.679213007128961539e-1_f64 * t6268 - 0.34930954652346593433e-1_f64 * t7961 + t7968 - 0.54878743191129263322e-2_f64 * t9320 - 0.97574405393827830187e-2_f64 * t9323 - 0.43341108700271342816e-1_f64 * t9327 * t576 - 0.42683466926433871473e0_f64 * t8026 - t8046;
    (t9327, t9331)
}
