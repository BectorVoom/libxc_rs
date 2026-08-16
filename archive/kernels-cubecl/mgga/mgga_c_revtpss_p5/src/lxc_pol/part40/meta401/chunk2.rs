//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1477/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1477<F: Float>(t10208: F, t69: F, t2195: F, t2289: F, t31027: F, t8312: F, t31032: F, t8316: F, t2340: F, t8311: F, t661: F, t665: F) -> (F, F, F, F, F, F) {
    let t31035 = t69 * t10208;
    let t31134 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t2289 * t2195;
    let t31135 = t31027 * t8312;
    let t31137 = t31032 * t8316;
    let t31139 = t8311 * t2340;
    let t31142 = t665 * t661;
    (t31035, t31134, t31135, t31137, t31139, t31142)
}
