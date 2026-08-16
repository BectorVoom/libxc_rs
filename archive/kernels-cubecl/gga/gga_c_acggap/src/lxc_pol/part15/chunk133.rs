//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 133/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk133<F: Float>(t150: F, t198: F, t222: F, t226: F, t231: F, t245: F, t278: F, t280: F, t285: F, t290: F, t167: F, t94: F) -> (F, F) {
    let t400 = (t198 + t222 + t226 - t231 + t245 + t278 + t280 - t285 - t290) * t150;
    let t402 = t94 * t167;
    (t400, t402)
}
