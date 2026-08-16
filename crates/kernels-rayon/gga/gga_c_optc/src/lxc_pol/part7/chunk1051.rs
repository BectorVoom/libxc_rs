//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1051/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1051(t135: f64, t141: f64, t161: f64, t2011: f64, t2021: f64, t22777: f64, t22781: f64, t22786: f64, t22788: f64, t22792: f64, t22798: f64, t22800: f64, t22807: f64, t22811: f64, t22815: f64, t22819: f64, t22822: f64, t22827: f64, t22830: f64, t22832: f64, t22838: f64, t22841: f64, t3439: f64, t3440: f64, t629: f64, t645: f64, t660: f64, t6781: f64, t6876: f64, t6945: f64, t9600: f64, t9678: f64, t9771: f64) -> f64 {
    let t22843 = 0.21732903724471894636e0_f64 * t3439 * t3440 * t22777 + 0.38032581517825815613e-1_f64 * t2021 * t161 * t22781 + 0.65198711173415683912e-1_f64 * t22786 * t161 * t22788 - 0.97798066760123525865e-1_f64 * t6876 * t161 * t22792 + 0.1915715217194189231e1_f64 * t22798 - 0.43103592386869257697e0_f64 * t22800 - 0.65198711173415683908e0_f64 * t9678 * t660 * t6781 - 0.26079484469366273564e0_f64 * t9600 * t141 * t645 * t22807 - 3.0_f64 / 2.0_f64 * t6945 * t629 * t22811 + t2011 * t629 * t22815 / 4.0_f64 - 0.30426065214260652492e1_f64 * t22819 - 0.19559613352024705172e1_f64 * t3439 * t9771 * t22822 + 0.16299677793353920977e0_f64 * t135 * t22827 + 0.86207184773738515393e1_f64 * t22830 + 0.60852130428521304982e1_f64 * t22832 + 0.22819548910695489368e1_f64 * t135 * t22838 - 7.0_f64 / 4.0_f64 * t22841;
    t22843
}
