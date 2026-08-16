//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 606/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk606(t313: f64, t4963: f64, t2679: f64, t4942: f64, t2596: f64, t4561: f64, t894: f64, t2648: f64, t4565: f64, t897: f64, t2581: f64, t2622: f64, t2640: f64, t2668: f64, t2678: f64, t3606: f64, t3632: f64, t3830: f64, t4930: f64, t4934: f64, t4938: f64, t4943: f64, t4948: f64, t862: f64, t874: f64, t893: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4964 = t313 * t4963;
    let t4967 = t4942 * t2679;
    let t4971 = t2596 * t4561;
    let t4972 = t894 * t4971;
    let t4975 = t2648 * t4561;
    let t4976 = t894 * t4975;
    let t4979 = t897 * t4565;
    let t4980 = t894 * t4979;
    let t4983 = -t2581 + t3606 / 432.0_f64 + t862 * t4930 / 216.0_f64 - t862 * t4934 / 144.0_f64 + t862 * t4938 / 288.0_f64 + 0.9157278480459830169e1_f64 * t2668 * t4943 + 0.47333755318775392234e-1_f64 * t3632 + 0.47333755318775392234e-1_f64 * t2640 * t4948 + 0.35500316489081544176e-1_f64 * t874 * t4964 - 0.45786392402299150845e1_f64 * t2678 * t4967 - t2622 + 0.24147670804968771818e-2_f64 * t3830 + 0.30184588506210964773e-2_f64 * t893 * t4972 - 0.36221506207453157728e-2_f64 * t893 * t4976 + 0.18110753103726578864e-2_f64 * t893 * t4980;
    (t4971, t4972, t4975, t4976, t4979, t4980, t4983)
}
