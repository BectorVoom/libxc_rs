//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 34/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk34(t6: f64, t73: f64, t69: f64, t63: f64, t66: f64, t21: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74 = t6 * t73;
    let t75 = t69 * t74;
    let t78 = 1.0_f64 + t63 * t66 * t75 / 96.0_f64;
    let t79 = f64::ln(t78);
    let t81 = 1.0_f64 + 0.66725e-1_f64 * t79;
    let t82 = 1.0_f64 / t81;
    let t84 = 1.0_f64 / t21;
    let t85 = t2 * t84;
    (t74, t75, t78, t81, t82, t85)
}
