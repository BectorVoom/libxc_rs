//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 901/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk901<F: Float>(t373: F, t1040: F, t2942: F, t376: F, t383: F, t3145: F, t56: F, t8429: F, t11: F) -> (F, F, F, F, F, F, F) {
    let t8611 = F::new(1.0)/pow_3_2::<f64>(t373);
    let t8612 = t2942 * t1040;
    let t8613 = t8611 * t8612;
    let t8617 = F::new(1.0) / t376 / t383 / F::new(4.0);
    let t8618 = t8617 * t8612;
    let t8620 = t56 * t3145;
    let t8621 = t8620 * t8429;
    let t8622 = t11 * t8621;
    (t8611, t8613, t8617, t8618, t8620, t8621, t8622)
}
