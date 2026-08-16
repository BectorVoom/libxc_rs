//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1066/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1066(t2089: f64, t7061: f64, t2182: f64, t6919: f64, t10004: f64, t116: f64, t2124: f64, t2126: f64, t22052: f64, t22811: f64, t22838: f64, t22865: f64, t22879: f64, t23052: f64, t23128: f64, t23136: f64, t23143: f64, t23149: f64, t23151: f64, t3501: f64, t627: f64, t686: f64, t705: f64) -> f64 {
    let t23153 = t7061 * t2089;
    let t23155 = t2182 * t6919;
    let t23159 = -0.10882232163006666614e1_f64 * t3501 * t22865 - 0.33855833396020740576e1_f64 * t23128 + 0.69545291918310062836e0_f64 * t2124 * t2126 * t22879 + t23136 - 0.1251815254529581131e2_f64 * t686 * t10004 * t22811 + 0.20863587575493018851e1_f64 * t2124 * t2126 * t23052 - 0.47962430644362715816e1_f64 * t23143 - 0.17386322979577515709e0_f64 * t686 * t627 * t116 * t22052 + 0.5642638899336790096e0_f64 * t23149 + 0.16227234780939014661e1_f64 * t23151 + 0.23981215322181357908e2_f64 * t23153 + 0.16927916698010370288e2_f64 * t23155 + 0.63479687617538888581e1_f64 * t705 * t22838;
    t23159
}
