//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 525/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk525(t3330: f64, t3331: f64, t1171: f64, t1175: f64, t1170: f64, t1169: f64, t284: f64) -> (f64, f64, f64, f64) {
    let t3333 = 2.0_f64 * t3330 * t3331;
    let t3334 = t1175 * t1171;
    let t3335 = t1170 * t3334;
    let t3337 = t1169 * t284;
    (t3333, t3334, t3335, t3337)
}
