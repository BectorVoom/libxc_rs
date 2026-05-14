//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 989/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk989<F: Float>(t1165: F, t30856: F, t35324: F, t604: F, t33751: F, t7413: F, t1181: F, t599: F, t31105: F, t35287: F, t35291: F, t35294: F, t35298: F, t35302: F, t35305: F, t35308: F, t35309: F, t35311: F, t35316: F, t35318: F, t35319: F, t35321: F) -> (F,) {
    let t35327 = t30856 * t1165 * t604 * t35324;
    let t35331 = t7413 * t1165 * t604 * t33751;
    let t35335 = t30856 * t1181 * t599 * t35324;
    let t35337 = 0.3773771074752498248e-2 * t31105 - t35287 + t35291 + 0.62896184579208304136e-3 * t35294 - 0.12862205435420921092e-2 * t35298 + t35302 + 0.53592522647587171215e-3 * t35305 - t35308 + 0.68598428988911579156e-2 * t35309 + 0.34299214494455789578e-2 * t35311 - t35316 - t35318 + 0.17149607247227894789e-2 * t35319 - 0.68598428988911579156e-2 * t35321 + 0.94344276868812456204e-3 * t35327 - 0.94344276868812456204e-3 * t35331 - 0.64311027177104605458e-3 * t35335;
    (t35337,)
}
