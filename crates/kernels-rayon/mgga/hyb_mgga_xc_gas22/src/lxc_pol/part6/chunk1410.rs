//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1410/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1410(t132: f64, t10325: f64, t11204: f64, t11209: f64, t1793: f64, t2002: f64, t2028: f64, t21911: f64, t25930: f64, t2688: f64, t2750: f64, t29765: f64, t3627: f64, t3925: f64, t3938: f64, t457: f64, t5891: f64, t675: f64, t7292: f64, t9354: f64, zeta_threshold: f64) -> f64 {
    let t133 = t132 <= zeta_threshold;
    let t30501 = piecewise3(t133, 0.0_f64, 40.0_f64 / 81.0_f64 * t21911 * t3925 * t2028 + 64.0_f64 / 27.0_f64 * t9354 * t29765 - 8.0_f64 / 27.0_f64 * t11204 * t2002 + 32.0_f64 / 9.0_f64 * t2688 * t457 * t2750 - 16.0_f64 / 9.0_f64 * t3627 * t1793 + 16.0_f64 / 3.0_f64 * t3627 * t5891 - 8.0_f64 / 27.0_f64 * t7292 * t3938 * t2028 + 8.0_f64 / 9.0_f64 * t2688 * t10325 * t675 + 4.0_f64 / 9.0_f64 * t11209 * t2002 - t25930);
    t30501
}
