//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 941/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk941<F: Float>(t9513: F, t9516: F, t9518: F, t9521: F, t9523: F, t9526: F, t9530: F, t9533: F, t9536: F, t9539: F, t9541: F, t9544: F, t9546: F) -> F {
    let t10856 = F::cast_from(0.12974218172834570556e-1_f64) * t9513 + F::cast_from(0.27801896084645508334e-2_f64) * t9516 + F::cast_from(0.55603792169291016668e-2_f64) * t9518 - F::cast_from(0.14492726735651760868e-5_f64) * t9521 - F::cast_from(0.10136107947527008247e-3_f64) * t9523 - F::cast_from(0.10136107947527008247e-3_f64) * t9526 + F::cast_from(0.30361328125000000002e-3_f64) * t9530 - F::cast_from(0.10120442708333333334e-3_f64) * t9533 + F::cast_from(0.6746961805555555556e-5_f64) * t9536 + F::cast_from(0.28985453471303521736e-5_f64) * t9539 + F::cast_from(0.2471588561924985691e-3_f64) * t9541 + F::cast_from(0.2471588561924985691e-3_f64) * t9544 - F::cast_from(0.6746961805555555556e-5_f64) * t9546;
    t10856
}
