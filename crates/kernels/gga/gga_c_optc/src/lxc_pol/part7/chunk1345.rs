//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1345/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1345<F: Float>(t26309: F, t26311: F, t26314: F, t26319: F, t26324: F, t26326: F, t26328: F, t26330: F, t26332: F, t26339: F, t26343: F, t26836: F) -> F {
    let t26846 = F::new(0.47488888888888888888e-1) * t26309 - F::new(0.94977777777777777776e-1) * t26311 + t26836 + F::new(0.23744444444444444444e-1) * t26314 + F::new(0.14246666666666666667e0) * t26319 - F::new(0.47488888888888888888e-1) * t26324 - F::new(0.47488888888888888888e-1) * t26326 - F::new(0.31659259259259259258e-1) * t26328 + F::new(0.94977777777777777776e-1) * t26330 + F::new(0.73871604938271604937e-1) * t26332 - F::new(0.52765432098765432099e-1) * t26339 - F::new(0.17808333333333333333e-1) * t26343;
    t26846
}
