//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 822/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk822(t7420: f64, t9838: f64, t291: f64, t8785: f64, t1734: f64, t1084: f64, t2546: f64, t3328: f64, t4: f64, t5: f64) -> (f64, f64, f64, f64) {
    let t9839 = t9838 * t7420;
    let t9841 = t8785 * t291;
    let t9842 = t1734 * t9841;
    let t9843 = t1084 * t9842;
    let t9846 = t2546 * t5 * t3328 * t4;
    (t9839, t9841, t9843, t9846)
}
