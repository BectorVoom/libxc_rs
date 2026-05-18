//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 942/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk942<F: Float>(t9513: F, t9516: F, t9518: F, t9521: F, t9523: F, t9526: F, t9530: F, t9533: F, t9536: F, t9539: F, t9541: F, t9544: F, t9546: F) -> F {
    let t10856 = F::new(0.12974218172834570556e-1) * t9513 + F::new(0.27801896084645508334e-2) * t9516 + F::new(0.55603792169291016668e-2) * t9518 - F::new(0.14492726735651760868e-5) * t9521 - F::new(0.10136107947527008247e-3) * t9523 - F::new(0.10136107947527008247e-3) * t9526 + F::new(0.30361328125000000002e-3) * t9530 - F::new(0.10120442708333333334e-3) * t9533 + F::new(0.6746961805555555556e-5) * t9536 + F::new(0.28985453471303521736e-5) * t9539 + F::new(0.2471588561924985691e-3) * t9541 + F::new(0.2471588561924985691e-3) * t9544 - F::new(0.6746961805555555556e-5) * t9546;
    t10856
}
