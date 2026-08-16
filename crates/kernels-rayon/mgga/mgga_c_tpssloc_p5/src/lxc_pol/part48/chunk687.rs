//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 687/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk687(t22674: f64, t6907: f64, t6897: f64, t12030: f64, t12444: f64, t1375: f64, t1386: f64, t2016: f64, t22622: f64, t22624: f64, t22630: f64, t22639: f64, t22646: f64, t22650: f64, t22653: f64, t22656: f64, t22664: f64, t22668: f64, t22670: f64, t3882: f64, t3912: f64, t568: f64, t6958: f64, t6963: f64, t6993: f64) -> (f64, f64) {
    let t22675 = t22674 * t6907;
    let t22676 = t6897 * t22675;
    let t22680 = t22622 * t568 + 2.0_f64 * t22624 * t568 + 4.0_f64 * t3882 * t6963 - 6.0_f64 * t1375 * t22630 + 0.3289868133696452873e-1_f64 * t22639 - t22646 + 0.82246703342411321825e-2_f64 * t22650 + 4.0_f64 * t1375 * t22653 - 2.0_f64 * t22656 * t1386 - 2.0_f64 * t12444 * t2016 - t6958 * t3912 - 0.82246703342411321825e-2_f64 * t22664 - 0.16449340668482264365e-1_f64 * t22668 - 2.0_f64 * t22670 * t1386 - t12030 * t2016 + 0.82246703342411321824e-2_f64 * t22676 - 2.0_f64 * t3882 * t6993;
    (t22676, t22680)
}
