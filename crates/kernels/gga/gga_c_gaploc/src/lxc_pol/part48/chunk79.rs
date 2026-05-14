//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 79/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk79<F: Float>(t326: F, t61: F, t315: F, t317: F, t323: F, t31: F, t4: F, t79: F) -> (F, F, F, F) {
    let t327 = t61 * t326;
    let t330 = 1.0 + 0.35750489951850426669e0 * t315 * t317 - 0.11502877786176224903e1 * t323 * t327;
    let t331 = 1.0 / t330;
    let t337 = 0.11073577833333333333e-2 * t4 * t79 * t31;
    (t327, t330, t331, t337)
}
