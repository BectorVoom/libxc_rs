//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 205/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk205<F: Float>(t760: F, t762: F, t206: F, t262: F, t78: F, t81: F, t212: F, t251: F, t225: F, t257: F) -> (F, F, F, F, F, F) {
    let t764 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t762;
    let t765 = t206 * t262;
    let t766 = F::cast_from(1.0_f64) / t78;
    let t770 = F::cast_from(1.0_f64) / t81;
    let t779 = t212 * t251;
    let t780 = t225 * t257;
    (t764, t765, t766, t770, t779, t780)
}
