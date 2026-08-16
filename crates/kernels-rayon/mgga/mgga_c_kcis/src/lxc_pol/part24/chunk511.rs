//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 511/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk511(t2919: f64, t2947: f64, t4612: f64, t4615: f64, t4618: f64, t4623: f64) -> f64 {
    let t4625 = t2947 + t2919 / 9.0_f64 + t4612 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4615 + 2.0_f64 / 3.0_f64 * t4618 - 2.0_f64 / 3.0_f64 * t4623;
    t4625
}
