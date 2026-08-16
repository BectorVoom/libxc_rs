//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3428/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3428<F: Float>(t41330: F, t41332: F, t52047: F, t52049: F, t52051: F, t63399: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63462: F, t63464: F) -> F {
    let t64400 = F::cast_from(0.15829629629629629629e-1_f64) * t52047 + F::cast_from(0.79148148148148148147e-2_f64) * t52049 + F::cast_from(0.13191358024691358025e-1_f64) * t52051 - F::cast_from(0.4274e0_f64) * t63399 - F::cast_from(0.79148148148148148147e-2_f64) * t41330 - F::cast_from(0.52765432098765432098e-2_f64) * t41332 + F::cast_from(0.11872222222222222222e-1_f64) * t63447 - F::cast_from(0.17808333333333333333e-1_f64) * t63451 - F::cast_from(0.52765432098765432097e-2_f64) * t63453 - F::cast_from(0.23744444444444444444e-1_f64) * t63457 + F::cast_from(0.15829629629629629629e-1_f64) * t63459 + F::cast_from(0.71233333333333333332e-1_f64) * t63462 - F::cast_from(0.79148148148148148146e-2_f64) * t63464;
    t64400
}
