//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1103/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1103<F: Float>(t10841: F, t2842: F, t10776: F, t10810: F, t2574: F, t10708: F, t10710: F, t24912: F, t2183: F, t37754: F, t2195: F, t37769: F, t7606: F) -> (F, F, F, F, F, F) {
    let t39908 = t10841 * t2842;
    let t39911 = t10776 * t10810 * t2574;
    let t39912 = F::cast_from(0.23115257973478049502e0_f64) * t39911;
    let t39920 = t10708 * t10710 * t24912;
    let t39922 = t2183 * t37754;
    let t39935 = t2195 * t37754;
    let t39939 = t37769 * t7606;
    (t39908, t39912, t39920, t39922, t39935, t39939)
}
