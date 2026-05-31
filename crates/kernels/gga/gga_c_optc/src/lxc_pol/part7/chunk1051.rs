//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1051/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1051<F: Float>(t135: F, t141: F, t161: F, t2011: F, t2021: F, t22777: F, t22781: F, t22786: F, t22788: F, t22792: F, t22798: F, t22800: F, t22807: F, t22811: F, t22815: F, t22819: F, t22822: F, t22827: F, t22830: F, t22832: F, t22838: F, t22841: F, t3439: F, t3440: F, t629: F, t645: F, t660: F, t6781: F, t6876: F, t6945: F, t9600: F, t9678: F, t9771: F) -> F {
    let t22843 = F::cast_from(0.21732903724471894636e0_f64) * t3439 * t3440 * t22777 + F::cast_from(0.38032581517825815613e-1_f64) * t2021 * t161 * t22781 + F::cast_from(0.65198711173415683912e-1_f64) * t22786 * t161 * t22788 - F::cast_from(0.97798066760123525865e-1_f64) * t6876 * t161 * t22792 + F::cast_from(0.1915715217194189231e1_f64) * t22798 - F::cast_from(0.43103592386869257697e0_f64) * t22800 - F::cast_from(0.65198711173415683908e0_f64) * t9678 * t660 * t6781 - F::cast_from(0.26079484469366273564e0_f64) * t9600 * t141 * t645 * t22807 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6945 * t629 * t22811 + t2011 * t629 * t22815 / F::cast_from(4.0_f64) - F::cast_from(0.30426065214260652492e1_f64) * t22819 - F::cast_from(0.19559613352024705172e1_f64) * t3439 * t9771 * t22822 + F::cast_from(0.16299677793353920977e0_f64) * t135 * t22827 + F::cast_from(0.86207184773738515393e1_f64) * t22830 + F::cast_from(0.60852130428521304982e1_f64) * t22832 + F::cast_from(0.22819548910695489368e1_f64) * t135 * t22838 - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t22841;
    t22843
}
