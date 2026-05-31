//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1488/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1488<F: Float>(t116926: F, t8312: F, t116929: F, t8316: F, t10241: F, t104: F, t46089: F, t655: F, t10199: F, t2339: F, t31027: F, t31430: F) -> (F, F, F, F, F, F) {
    let t117184 = t116926 * t8312;
    let t117186 = t116929 * t8316;
    let t117218 = t104 * t10241;
    let t117461 = t46089 * t655;
    let t117544 = t10199 * t2339;
    let t117918 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t31027 * t31430;
    (t117184, t117186, t117218, t117461, t117544, t117918)
}
