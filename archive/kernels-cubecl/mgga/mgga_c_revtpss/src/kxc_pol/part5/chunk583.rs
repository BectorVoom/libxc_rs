//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 583/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk583<F: Float>(t3109: F, t906: F, t247: F, t1063: F, t1086: F, t994: F, t3090: F) -> (F, F, F, F) {
    let t3110 = t3109 * t906;
    let t3111 = t247 * t3110;
    let t3112 = t1063 * t3111;
    let t3114 = t994 * t1086;
    let t3115 = t3114 * t3090;
    (t3111, t3112, t3114, t3115)
}
