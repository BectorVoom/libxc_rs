//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 871/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk871<F: Float>(t39373: F, t8620: F, t321: F, t3351: F, t511: F, t7248: F, t8502: F, t333: F, t7231: F, t880: F, t2339: F, t638: F, t7184: F) -> (F, F, F, F) {
    let t39374 = t8620 * t39373;
    let t39379 = t3351 * t7248 * t511 * t8502 * t321;
    let t39384 = t3351 * t7231 * t880 * t8502 * t333;
    let t39388 = t638 * t7184 * t2339;
    (t39374, t39379, t39384, t39388)
}
