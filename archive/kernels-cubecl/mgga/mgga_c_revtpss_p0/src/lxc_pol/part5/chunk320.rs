//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 320/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk320<F: Float>(t1044: F, t1045: F, t1042: F, t362: F, t39: F, t40: F) -> (F, F, F) {
    let t1046 = t1044 * t1045;
    let t1047 = t1042 * t1046;
    let t1050 = t362 * t39;
    let t1052 = F::cast_from(1.0_f64) / t40 / t1050;
    (t1046, t1047, t1052)
}
