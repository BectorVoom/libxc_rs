//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1203/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1203(t22252: f64, t22276: f64, t22344: f64, t22396: f64, t462: f64, t468: f64, t1156: f64, t7692: f64, t7768: f64, t1166: f64, t9666: f64, t2938: f64, t537: f64) -> (f64, f64, f64, f64, f64) {
    let t22400 = t462 * t468 * (t22252 + t22276 + t22344 + t22396);
    let t22506 = t7692 * t1156;
    let t22512 = t7768 * t1156;
    let t22531 = t1166 * t9666;
    let t22600 = t2938 * t537;
    (t22400, t22506, t22512, t22531, t22600)
}
