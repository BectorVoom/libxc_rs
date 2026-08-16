//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1257/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1257(t1796: f64, t3362: f64, t15056: f64, t377: f64, t283: f64, t5164: f64, t7755: f64, t1200: f64, t13181: f64, t5082: f64, t982: f64, t7749: f64) -> (f64, f64, f64, f64, f64) {
    let t95317 = t1796 * t3362;
    let t95319 = t15056 * t377;
    let t95321 = t5164 * t283;
    let t95322 = t95321 * t7755;
    let t95324 = t13181 * t1200;
    let t95326 = t5082 * t982;
    let t95327 = t95326 * t7749;
    (t95317, t95319, t95322, t95324, t95327)
}
