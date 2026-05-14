//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 810/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk810<F: Float>(t24: F, t5005: F, t10791: F, t1248: F, t1636: F, t10933: F, t3118: F, t353: F, t579: F, t609: F, t615: F, t606: F, t1701: F, t4857: F, t1848: F, t641: F, t916: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11003 = t24 * t5005;
    let t11013 = t1248 * t10791 * t1636;
    let t11030 = 0.93011851851851851854e0 * t10933;
    let t11032 = t353 * t3118 * t579;
    let t11033 = 0.73028148148148148147e0 * t11032;
    let t11036 = 1.0 / t609 / t615 / 8.0;
    let t11040 = 28.0 / 27.0 * t10933;
    let t11056 = 1.0/pow_3_2(t606);
    let t11091 = 0.93932222222222222223e0 * t10933;
    let t11092 = 0.73586666666666666667e0 * t11032;
    let t11105 = 0.55403703703703703703e-1 * t10933;
    let t11119 = t1701 * t4857;
    let t11153 = 1.0 / t641 / t916 / t1848;
    (t11003, t11013, t11030, t11033, t11036, t11040, t11056, t11091, t11092, t11105, t11119, t11153)
}
