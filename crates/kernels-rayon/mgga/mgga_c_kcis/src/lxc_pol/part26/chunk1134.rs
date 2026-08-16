//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1134/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1134(t28765: f64, t5654: f64, t6151: f64, t6188: f64, t7969: f64, t6176: f64) -> (f64, f64, f64, f64) {
    let t28766 = t28765 * t5654;
    let t28767 = t6151 * t28766;
    let t28771 = t7969 * t6188;
    let t28772 = t6176 * t28771;
    (t28766, t28767, t28771, t28772)
}
