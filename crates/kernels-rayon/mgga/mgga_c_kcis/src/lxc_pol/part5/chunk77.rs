//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 77/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk77(t169: f64, t234: f64, t5: f64, t7: f64, zeta_threshold: f64) -> (f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t235 = piecewise3(t170, zeta_threshold, t169);
    let t236 = t234 * t235;
    let t237 = t5 * t7;
    (t236, t237)
}
