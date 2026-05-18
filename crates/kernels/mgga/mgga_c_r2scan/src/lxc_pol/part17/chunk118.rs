//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 118/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk118<F: Float>(t364: F, t358: F, t245: F, t158: F) -> (F, F, F, F) {
    let t366 = F::new(1.0) - F::new(1.0) / t364;
    let t368 = t358 * t366 + F::new(1.0);
    let t369 = f64::ln(t368);
    let t371 = -F::new(0.285764e-1) * t245 + F::new(0.285764e-1) * t369;
    let t372 = t371 * t158;
    (t366, t368, t371, t372)
}
