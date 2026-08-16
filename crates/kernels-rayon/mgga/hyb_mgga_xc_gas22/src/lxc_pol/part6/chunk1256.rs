//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1256/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1256(t1159: f64, t22506: f64, t524: f64, t1143: f64, t9574: f64, t3756: f64, t7774: f64, t532: f64, t22512: f64, t536: f64, t15052: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26409 = t524 * t22506 * t1159;
    let t26416 = t1143 * t9574;
    let t26421 = t7774 * t3756;
    let t26425 = t7774 * t532;
    let t26429 = t536 * t22512 * t1159;
    let t26433 = t536 * t15052 * t531;
    (t26409, t26416, t26421, t26425, t26429, t26433)
}
