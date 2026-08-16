//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 172/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk172(t571: f64, t574: f64) -> (f64, f64, f64, f64) {
    let t677 = 0.107924e1_f64 + 0.3964e-1_f64 * t574 + 0.123825e-1_f64 * t571;
    let t680 = 1.0_f64 + t574 * t677 / 2.0_f64;
    let t681 = t680 * t680;
    let t682 = 1.0_f64 / t681;
    (t677, t680, t681, t682)
}
