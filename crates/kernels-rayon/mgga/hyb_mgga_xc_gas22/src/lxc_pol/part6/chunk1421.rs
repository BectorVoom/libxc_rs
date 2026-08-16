//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1421/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1421(t1161: f64, t2893: f64, t4512: f64, t1117: f64, t11279: f64, t13638: f64, t26093: f64, t26096: f64, t26345: f64, t2821: f64, t2834: f64, t2838: f64, t30703: f64, t30723: f64, t30733: f64, t30736: f64, t30739: f64, t30748: f64, t3661: f64, t3663: f64, t7643: f64, t9440: f64, t9523: f64, t9535: f64, t9703: f64) -> (f64, f64) {
    let t30752 = t1161 * t4512 * t2893;
    let t30757 = -3200.0_f64 / 81.0_f64 * t3661 * t30723 + 88.0_f64 / 9.0_f64 * t7643 * t30703 + 800.0_f64 / 27.0_f64 * t9703 * t11279 + 4000.0_f64 * t26096 * t3663 * t13638 - 4000.0_f64 * t30733 * t9523 - 5600.0_f64 * t26093 * t30736 + 5600.0_f64 * t30739 * t9535 - 800.0_f64 / 3.0_f64 * t26345 * t30736 + 800.0_f64 / 3.0_f64 * t1117 * t9440 * t9535 + 88.0_f64 / 9.0_f64 * t2834 * t30748 - 88.0_f64 / 9.0_f64 * t2838 * t30752 + 88.0_f64 / 27.0_f64 * t2821 * t30748;
    (t30752, t30757)
}
