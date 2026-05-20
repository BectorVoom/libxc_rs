//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1920/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1920<F: Float>(t28399: F, t686: F, t72: F, t7058: F, t103000: F, t93371: F, t25410: F, t8011: F, t93240: F, t7064: F, t28447: F, t689: F, t887: F) -> (F, F, F, F, F) {
    let t103117 = t28399 * t72 * t686;
    let t103119 = F::cast_from(0.14456046980341999104e-1_f64) * t7058 * t103117;
    let t103122 = t93371 * t103000;
    let t103130 = t93240 * t25410 * t8011;
    let t103136 = F::cast_from(0.25702851531048074406e-1_f64) * t7064 * t103117;
    let t103140 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t28447 * t887;
    (t103119, t103122, t103130, t103136, t103140)
}
