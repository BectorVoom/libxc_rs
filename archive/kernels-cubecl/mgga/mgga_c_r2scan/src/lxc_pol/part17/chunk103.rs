//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 103/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk103<F: Float>(t122: F, t263: F, t299: F, t309: F, t313: F, param_eta: F) -> (F, F, F) {
    let t317 = param_eta * t122;
    let t320 = F::cast_from(3.0_f64) / F::cast_from(10.0_f64) * t313 * (t299 + t309) + t317 * t263 / F::cast_from(8.0_f64);
    let t321 = F::cast_from(1.0_f64) / t320;
    (t317, t320, t321)
}
