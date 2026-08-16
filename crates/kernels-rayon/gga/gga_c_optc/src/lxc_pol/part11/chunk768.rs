//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 768/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk768(t11899: f64, t2849: f64, t115: f64, t1497: f64, t2770: f64, t3209: f64, t1724: f64, t1540: f64, t7878: f64, t1170: f64, t1528: f64, t7274: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12568 = t11899 * t2849;
    let t12577 = t1497 * t2770 * t115;
    let t12578 = t3209 * t12577;
    let t12581 = t1724 * t12577;
    let t12594 = t7878 * t1540;
    let t12595 = t1170 * t12594;
    let t12597 = t7274 * t1528;
    (t12568, t12578, t12581, t12594, t12595, t12597)
}
