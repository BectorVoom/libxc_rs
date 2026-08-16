//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 687/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk687(t7615: f64, t7618: f64, t7620: f64, t7622: f64, t7625: f64, t7628: f64) -> f64 {
    let t7669 = 0.1875e0_f64 * t7615 - 0.1875e0_f64 * t7618 - 0.375e0_f64 * t7620 - 0.809375e-1_f64 * t7622 + 0.809375e-1_f64 * t7625 + 0.32375e0_f64 * t7628;
    t7669
}
