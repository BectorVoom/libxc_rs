//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1010/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1010(t2893: f64, t3771: f64, t1539: f64, t7692: f64, t1535: f64, t2880: f64, t2876: f64, t2869: f64, t1145: f64, t1530: f64, t2884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9419 = t3771 * t2893;
    let t9436 = t7692 * t1539;
    let t9440 = t2880 * t1535;
    let t9441 = t9440 * t2876;
    let t9444 = t1539 * t2869;
    let t9448 = t1539 * t2876;
    let t9449 = t1145 * t9448;
    let t9452 = t1535 * t2876;
    let t9453 = t1145 * t9452;
    let t9458 = t2884 * t1530;
    (t9419, t9436, t9440, t9441, t9444, t9448, t9449, t9453, t9458)
}
