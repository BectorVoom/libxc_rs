//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 209/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk209<F: Float>(t663: F, t666: F, t669: F, t673: F, t675: F, t678: F) -> F {
    let t680 = -F::cast_from(0.57538888888888888889e0_f64) * t663 + F::cast_from(0.11507777777777777778e1_f64) * t666 + F::cast_from(0.40256666666666666667e0_f64) * t669 + F::cast_from(0.366775e-1_f64) * t673 + F::cast_from(0.73355e-1_f64) * t675 + F::cast_from(0.137975e0_f64) * t678;
    t680
}
