//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 971/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk971(t11808: f64, t9419: f64, t11784: f64, t3789: f64, t190: f64, t932: f64, t11449: f64, t11804: f64, t7735: f64, t11781: f64, t3375: f64, t1084: f64, t11508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11809 = t11808 * t9419;
    let t11811 = t11784 * t3789;
    let t11813 = t932 * t190;
    let t11814 = t11813 * t11449;
    let t11815 = t11804 * t7735;
    let t11816 = t11814 * t11815;
    let t11818 = t11808 * t3789;
    let t11820 = t11781 * t3375;
    let t11822 = t1084 * t11508;
    (t11809, t11811, t11813, t11814, t11815, t11816, t11818, t11820, t11822)
}
