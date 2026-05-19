//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 103/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk103<F: Float>(t122: F, t263: F, t299: F, t309: F, t313: F, param_eta: F) -> (F, F, F) {
    let t317 = param_eta * t122;
    let t320 = F::new(3.0) / F::new(10.0) * t313 * (t299 + t309) + t317 * t263 / F::new(8.0);
    let t321 = F::new(1.0) / t320;
    (t317, t320, t321)
}
