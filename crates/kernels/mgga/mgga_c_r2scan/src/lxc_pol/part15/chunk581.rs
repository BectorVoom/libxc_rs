//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 581/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk581<F: Float>(t106: F, t2850: F, t797: F, t97: F, t986: F, t2266: F, t481: F, t104: F) -> (F, F, F, F) {
    let t2853 = t97 * t106 * t2850 * t797;
    let t2854 = t986 * t797;
    let t2856 = t2266 * t2854 * t481;
    let t2857 = F::cast_from(3.0_f64) * t2856;
    let t2858 = t97 * t104;
    (t2853, t2854, t2857, t2858)
}
