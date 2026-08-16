//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1239/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1239(t3418: f64, t6669: f64, t2314: f64, t8709: f64, t3385: f64, t6712: f64, t3352: f64, t6564: f64, t1370: f64, t6641: f64, t6667: f64, t1346: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24989 = t3418 * t6669;
    let t24996 = t8709 * t2314;
    let t25049 = t3385 * t6712;
    let t25116 = t3352 * t6564;
    let t25129 = t6641 * t1370;
    let t25132 = t6667 * t1370;
    let t25146 = t6579 * t1346;
    (t24989, t24996, t25049, t25116, t25129, t25132, t25146)
}
