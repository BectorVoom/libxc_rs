//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 590/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk590<F: Float>(t313: F, t4963: F, t2679: F, t4942: F, t2596: F, t4561: F, t894: F, t2648: F, t4565: F, t897: F, t2581: F, t2622: F, t2640: F, t2668: F, t2678: F, t3606: F, t3632: F, t3830: F, t4930: F, t4934: F, t4938: F, t4943: F, t4948: F, t862: F, t874: F, t893: F) -> (F, F, F, F, F, F, F) {
    let t4964 = t313 * t4963;
    let t4967 = t4942 * t2679;
    let t4971 = t2596 * t4561;
    let t4972 = t894 * t4971;
    let t4975 = t2648 * t4561;
    let t4976 = t894 * t4975;
    let t4979 = t897 * t4565;
    let t4980 = t894 * t4979;
    let t4983 = -t2581 + t3606 / 432.0 + t862 * t4930 / 216.0 - t862 * t4934 / 144.0 + t862 * t4938 / 288.0 + 0.9157278480459830169e1 * t2668 * t4943 + 0.47333755318775392234e-1 * t3632 + 0.47333755318775392234e-1 * t2640 * t4948 + 0.35500316489081544176e-1 * t874 * t4964 - 0.45786392402299150845e1 * t2678 * t4967 - t2622 + 0.24147670804968771818e-2 * t3830 + 0.30184588506210964773e-2 * t893 * t4972 - 0.36221506207453157728e-2 * t893 * t4976 + 0.18110753103726578864e-2 * t893 * t4980;
    (t4971, t4972, t4975, t4976, t4979, t4980, t4983)
}
