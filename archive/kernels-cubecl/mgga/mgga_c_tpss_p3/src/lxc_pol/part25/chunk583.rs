//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 583/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk583<F: Float>(t1215: F, t3240: F, t159: F, t527: F, t210: F, t1218: F, t521: F) -> (F, F, F, F) {
    let t3241 = t3240 * t1215;
    let t3243 = t159 * t527;
    let t3244 = t210 * t3243;
    let t3255 = F::cast_from(1.0_f64) / t1218 / t521;
    (t3241, t3243, t3244, t3255)
}
