//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1322/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1322<F: Float>(t22502: F, t370: F, t376: F, t26267: F, t2942: F, t2950: F, t8611: F, t8647: F, t8673: F, t8617: F, t8644: F, t25: F, t26287: F, t2869: F) -> (F, F, F, F, F, F) {
    let t26424 = F::new(1.0) / t376 / t22502 / t370 / F::new(96.0);
    let t26425 = t26424 * t26267;
    let t26428 = t8611 * t2942 * t2950;
    let t26430 = t8647 * t8673;
    let t26433 = t8617 * t2942 * t2950;
    let t26435 = t8644 * t8673;
    let t26443 = t25 * t2869 * t26287;
    (t26425, t26428, t26430, t26433, t26435, t26443)
}
