//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 761/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk761(t24: f64, t2887: f64, t2877: f64, t984: f64, t2810: f64, t296: f64) -> (f64, f64, f64) {
    let t9959 = t24 * t2887;
    let t9970 = t984 * t2877;
    let t9985 = 1.0_f64 / t2810 / t296;
    (t9959, t9970, t9985)
}
