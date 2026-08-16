//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 99/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk99<F: Float>(t296: F, t298: F, rho1: F, tau1: F) -> (F, F, F) {
    let t299 = t298 * t296;
    let t301 = pow_1_3::<F>(rho1);
    let t302 = t301 * t301;
    let t304 = F::cast_from(1.0_f64) / t302 / rho1;
    let t305 = tau1 * t304;
    (t299, t302, t305)
}
