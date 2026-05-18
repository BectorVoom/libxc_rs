//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1226/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1226<F: Float>(t10986: F, t40713: F, t10635: F, t40282: F, t38323: F, t38334: F, t38337: F, t38339: F, t38342: F, t38347: F, t38350: F, t38356: F, t38359: F, t38363: F, t40699: F, t40704: F, t40708: F, t40711: F) -> (F, F, F) {
    let t40715 = F::new(5.0) / F::new(8.0) * t40713 * t10986;
    let t40717 = F::new(15.0) / F::new(8.0) * t40282 * t10635;
    let t40718 = t38323 - F::new(0.15243824895787514157e-3) * t38334 + t38337 + F::new(0.16260079888840015101e-2) * t38339 - t38342 + t38347 - t38350 - t40699 - F::new(0.38422568777328955684e-2) * t38356 + F::new(0.60975299583150056628e-3) * t38359 + t38363 + t40704 + t40708 - t40711 - t40715 + t40717;
    (t40715, t40717, t40718)
}
