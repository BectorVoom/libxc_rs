//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 209/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk209<F: Float>(t195: F, t288: F, t656: F, t19: F, t355: F, t20: F, t5: F, t351: F, t123: F, t203: F, t202: F, t6: F) -> (F, F, F, F, F, F, F, F, F) {
    let t657 = t195 * t288;
    let t658 = t656 * t657;
    let t659 = F::cast_from(0.10843581300301739842e-1_f64) * t658;
    let t660 = t355 * t19;
    let t661 = t20 * t5;
    let t662 = t661 * t351;
    let t663 = t660 * t662;
    let t665 = t203 * t123;
    let t666 = t202 * t665;
    let t668 = t6 * t123;
    (t657, t659, t660, t661, t662, t663, t665, t666, t668)
}
