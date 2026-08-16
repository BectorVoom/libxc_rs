//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1401/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1401(t1007: f64, t1014: f64, t10873: f64, t11003: f64, t11008: f64, t25427: f64, t2576: f64, t2609: f64, t30205: f64, t30207: f64, t30209: f64, t30211: f64, t30213: f64, t30215: f64, t30221: f64, t3591: f64, t3597: f64, t3606: f64, t9064: f64, t9274: f64, t9296: f64) -> f64 {
    let t30366 = t30205 + t30207 + t30209 - t30211 + t30213 + t30215 + 0.23392894490538584828e1_f64 * t1014 * t2576 * t11003 * t1007 - 0.20508037716432813316e4_f64 * t2609 * t10873 - 0.70178683471615754484e1_f64 * t2609 * t11008 - 0.69263436422725855034e2_f64 * t9296 * t3606 + 0.46785788981077169656e1_f64 * t3591 * t9274 + 0.46785788981077169656e1_f64 * t9296 * t3597 - 0.41016075432865626631e4_f64 * t25427 * t9064 * t30221;
    t30366
}
