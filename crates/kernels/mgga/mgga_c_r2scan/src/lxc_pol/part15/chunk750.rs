//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 750/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk750<F: Float>(t6064: F, t6086: F, t6093: F, t2086: F, t776: F, t23: F, t271: F) -> (F, F, F) {
    let t6094 = t6086 * t6064;
    let t6095 = t6093 * t6094;
    let t6097 = t776 * t2086;
    let t6100 = F::cast_from(1.0_f64) / t23 / t271;
    (t6095, t6097, t6100)
}
