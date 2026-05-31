//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1758/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1758<F: Float>(t159: F, t2698: F, t218: F, t816: F, t228: F, t7021: F, t802: F, t7043: F, t826: F, t2736: F, t7082: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t25273 = t2698 * t159;
    let t25275 = t25273 * t218 * t816;
    let t25276 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t25275;
    let t25277 = t7021 * t228;
    let t25278 = t25277 * t802;
    let t25282 = t7043 * t826;
    let t25283 = t2736 * t25282;
    let t25284 = F::cast_from(0.50820002809285328225e-5_f64) * t25283;
    let t25295 = t7082 * t72;
    (t25273, t25276, t25277, t25278, t25282, t25284, t25295)
}
