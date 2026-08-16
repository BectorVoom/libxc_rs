//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1114/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1114(t3235: f64, t3237: f64, t5939: f64, t179: f64, t3026: f64, t404: f64, t6380: f64, t1184: f64, t2240: f64, t237: f64, t6323: f64, t1208: f64) -> (f64, f64, f64, f64, f64) {
    let t22469 = t3235 * t5939 * t3237;
    let t22474 = t404 * t179 * t6380 * t3026;
    let t22475 = 0.28582678745379824648e-3_f64 * t22474;
    let t22500 = t2240 * t1184;
    let t22503 = t237 * t6323;
    let t22561 = t6323 * t1208;
    (t22469, t22475, t22500, t22503, t22561)
}
