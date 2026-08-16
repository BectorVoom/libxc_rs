//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 876/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk876<F: Float>(t30401: F, t30402: F, t322: F, t7325: F, t151: F, t30400: F) -> (F, F) {
    let t30405 = t30401 * t30402 * t7325 * t322;
    let t30406 = F::cast_from(0.12862205435420921092e-2_f64) * t30405;
    let t30407 = t151 * t30400;
    (t30406, t30407)
}
