//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 871/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk871<F: Float>(t7523: F, t7525: F, t7527: F, t7531: F, t7535: F, t7550: F, t7571: F, t7573: F, t7576: F, t7580: F, t7583: F, t8363: F) -> F {
    let t8364 = F::cast_from(0.22615185185185185185e4_f64) * t7523;
    let t8375 = -t8364 - F::cast_from(0.26222222222222222223e3_f64) * t7571 + F::cast_from(0.15733333333333333334e3_f64) * t7573 + F::cast_from(0.52444444444444444444e2_f64) * t7576 - F::cast_from(0.34962962962962962963e2_f64) * t7580 - F::cast_from(0.78666666666666666667e2_f64) * t7583 - F::cast_from(0.96922222222222222223e3_f64) * t7525 + F::cast_from(0.72691666666666666668e3_f64) * t7531 + F::cast_from(0.48461111111111111112e3_f64) * t7527 - F::cast_from(0.80768518518518518518e3_f64) * t7535 - F::cast_from(0.72691666666666666667e3_f64) * t7550;
    let t8376 = t8363 + t8375;
    t8376
}
