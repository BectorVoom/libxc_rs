//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 967/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk967<F: Float>(t22834: F, t136: F, t141: F, t22752: F, t6910: F, t6941: F, t135: F, t161: F, t2011: F, t2021: F, t22777: F, t22781: F, t22786: F, t22788: F, t22792: F, t22798: F, t22800: F, t22807: F, t22811: F, t22815: F, t22819: F, t22822: F, t22827: F, t22830: F, t22832: F, t3439: F, t3440: F, t629: F, t645: F, t660: F, t6781: F, t6876: F, t6945: F, t9600: F, t9678: F, t9771: F) -> (F, F) {
    let t22835 = 1.0 / t22834;
    let t22836 = t136 * t22835;
    let t22838 = t22836 * t141 * t22752;
    let t22841 = t6941 * t6910;
    let t22843 = 0.21732903724471894636e0 * t3439 * t3440 * t22777 + 0.38032581517825815613e-1 * t2021 * t161 * t22781 + 0.65198711173415683912e-1 * t22786 * t161 * t22788 - 0.97798066760123525865e-1 * t6876 * t161 * t22792 + 0.1915715217194189231e1 * t22798 - 0.43103592386869257697e0 * t22800 - 0.65198711173415683908e0 * t9678 * t660 * t6781 - 0.26079484469366273564e0 * t9600 * t141 * t645 * t22807 - 3.0 / 2.0 * t6945 * t629 * t22811 + t2011 * t629 * t22815 / 4.0 - 0.30426065214260652492e1 * t22819 - 0.19559613352024705172e1 * t3439 * t9771 * t22822 + 0.16299677793353920977e0 * t135 * t22827 + 0.86207184773738515393e1 * t22830 + 0.60852130428521304982e1 * t22832 + 0.22819548910695489368e1 * t135 * t22838 - 7.0 / 4.0 * t22841;
    (t22838, t22843)
}
