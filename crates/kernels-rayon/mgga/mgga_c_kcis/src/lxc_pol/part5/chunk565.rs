//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 565/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk565(t89: f64, t828: f64, t1018: f64, t341: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t3187 = 2.0_f64 * t89;
    let t3188 = 2.0_f64 * t828;
    let t3198 = t1018 * t341;
    let t3200 = t86 * t1017 * t3198;
    (t3187, t3188, t3198, t3200)
}
