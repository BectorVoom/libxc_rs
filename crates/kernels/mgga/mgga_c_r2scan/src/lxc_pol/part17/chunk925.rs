//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 925/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk925<F: Float>(t10710: F, t6481: F, t10708: F, t10707: F, t546: F) -> (F, F, F) {
    let t10711 = t10710 * t6481;
    let t10712 = t10708 * t10711;
    let t10713 = F::cast_from(0.14282990759302185292e-1_f64) * t10712;
    let t10728 = t546 * t10707;
    (t10711, t10713, t10728)
}
