//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1066/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1066<F: Float>(t2089: F, t7061: F, t2182: F, t6919: F, t10004: F, t116: F, t2124: F, t2126: F, t22052: F, t22811: F, t22838: F, t22865: F, t22879: F, t23052: F, t23128: F, t23136: F, t23143: F, t23149: F, t23151: F, t3501: F, t627: F, t686: F, t705: F) -> F {
    let t23153 = t7061 * t2089;
    let t23155 = t2182 * t6919;
    let t23159 = -F::new(0.10882232163006666614e1) * t3501 * t22865 - F::new(0.33855833396020740576e1) * t23128 + F::new(0.69545291918310062836e0) * t2124 * t2126 * t22879 + t23136 - F::new(0.1251815254529581131e2) * t686 * t10004 * t22811 + F::new(0.20863587575493018851e1) * t2124 * t2126 * t23052 - F::new(0.47962430644362715816e1) * t23143 - F::new(0.17386322979577515709e0) * t686 * t627 * t116 * t22052 + F::new(0.5642638899336790096e0) * t23149 + F::new(0.16227234780939014661e1) * t23151 + F::new(0.23981215322181357908e2) * t23153 + F::new(0.16927916698010370288e2) * t23155 + F::new(0.63479687617538888581e1) * t705 * t22838;
    t23159
}
