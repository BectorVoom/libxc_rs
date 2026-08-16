//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2332/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2332<F: Float>(t100822: F, t100864: F, t96749: F, t96793: F, t96840: F, t97814: F, t97859: F, t97906: F, t16524: F, t26545: F, t1873: F, t66958: F) -> (F, F, F) {
    let t100867 = t96749 + t96793 + t96840 + t97814 + t97859 + t97906 + t100822 + t100864;
    let t100871 = F::cast_from(54.0_f64) * t16524 * t26545;
    let t100873 = F::cast_from(0.135e2_f64) * t66958 * t1873;
    (t100867, t100871, t100873)
}
