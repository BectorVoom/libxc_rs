//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 200/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk200(t169: f64, t171: f64, t829: f64, zeta_threshold: f64) -> (f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t832 = piecewise3(t170, 0.0_f64, 4.0_f64 / 3.0_f64 * t171 * t829);
    let t833 = -t829;
    (t832, t833)
}
