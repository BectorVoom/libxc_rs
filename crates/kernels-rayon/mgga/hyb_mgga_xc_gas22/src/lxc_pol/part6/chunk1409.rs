//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1409/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1409(t7: f64, t11192: f64, t11197: f64, t1793: f64, t1796: f64, t1808: f64, t21896: f64, t25907: f64, t2680: f64, t2750: f64, t28813: f64, t3619: f64, t3804: f64, t3814: f64, t457: f64, t545: f64, t5891: f64, t7281: f64, t9340: f64, t9909: f64, zeta_threshold: f64) -> f64 {
    let t8 = t7 <= zeta_threshold;
    let t30477 = piecewise3(t8, 0.0_f64, 40.0_f64 / 81.0_f64 * t21896 * t3814 * t1808 - 64.0_f64 / 27.0_f64 * t9340 * t28813 - 8.0_f64 / 27.0_f64 * t11192 * t1796 + 32.0_f64 / 9.0_f64 * t2680 * t457 * t2750 + 16.0_f64 / 9.0_f64 * t3619 * t1793 - 16.0_f64 / 3.0_f64 * t3619 * t5891 - 8.0_f64 / 27.0_f64 * t7281 * t3804 * t1808 + 8.0_f64 / 9.0_f64 * t2680 * t9909 * t545 + 4.0_f64 / 9.0_f64 * t11197 * t1796 + t25907);
    t30477
}
