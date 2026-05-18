//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1175/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1175<F: Float>(t31697: F, t31702: F, t31704: F, t31721: F, t36063: F, t36066: F, t36068: F, t36070: F, t36072: F, t36075: F, t36077: F, t36082: F, t36083: F, t36086: F, t36088: F, t36090: F, t36093: F) -> F {
    let t36095 = t36063 / F::new(48.0) - t36066 + t36068 / F::new(64.0) + t36070 + F::new(0.53592522647587171215e-3) * t31697 - t36072 + F::new(0.31448092289604152068e-3) * t31702 + F::new(0.41930789719472202756e-3) * t31704 + t36075 + F::new(0.18868855373762491241e-2) * t36077 + t36082 - t31721 + F::new(0.21437009059034868486e-3) * t36083 + t36086 + t36088 - t36090 - F::new(0.47172138434406228102e-3) * t36093;
    t36095
}
