//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 686/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk686(t3443: f64, t3444: f64, t1379: f64, t2322: f64, t260: f64, t3315: f64, t3318: f64, t3320: f64, t3323: f64, t3355: f64, t3359: f64, t3397: f64, t3426: f64, t3430: f64, t3436: f64, t3440: f64, t856: f64, t858: f64) -> (f64, f64) {
    let t3445 = t3443 * t3444;
    let t3448 = -t3315 + t3318 + t3320 - t3323 + t3355 + t3359 + t260 * t3426 + 0.19751673498613801407e-1_f64 * t260 * t3397 - 0.5848223622634646207e0_f64 * t3430 * t858 - 0.5848223622634646207e0_f64 * t2322 * t1379 + 0.11696447245269292414e1_f64 * t856 * t3436 - 0.5848223622634646207e0_f64 * t856 * t3440 - 0.17315859105681463759e2_f64 * t856 * t3445;
    (t3445, t3448)
}
