//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1328/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1328<F: Float>(t2919: F, t8743: F, t1102: F, t8549: F, t8769: F, t26261: F, t26264: F, t26252: F, t26258: F, t26268: F, t26271: F, t26326: F, t26328: F, t26330: F, t26332: F, t26347: F, t26351: F, t26354: F, t26358: F) -> (F, F, F) {
    let t26490 = F::new(0.70178680769462448852e1) * t8743 * t2919;
    let t26493 = F::new(0.46785787179641632568e1) * t1102 * t8549 * t8769;
    let t26496 = F::new(0.31310740740740740741e1) * t26261;
    let t26497 = F::new(0.13490888888888888889e1) * t26264;
    let t26508 = F::new(0.44729629629629629629e0) * t26252 + F::new(0.40256666666666666666e1) * t26258 + t26496 + t26497 + F::new(0.6189328125e-1) * t26268 + F::new(0.247573125e0) * t26271 + F::new(0.258925e1) * t26347 - F::new(0.80513333333333333336e0) * t26326 - F::new(0.53675555555555555556e0) * t26328 - F::new(0.44152e0) * t26351 + F::new(0.44152e0) * t26354 + F::new(0.16102666666666666667e1) * t26330 + F::new(0.12524296296296296297e1) * t26332 + F::new(0.98115555555555555556e0) * t26358;
    (t26490, t26493, t26508)
}
