//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 243/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk243(t228: f64, t780: f64, t827: f64, t895: f64, t899: f64, t906: f64, t279: f64, sigma0: f64) -> (f64, f64) {
    let t908 = t228 * t895 - t899 * t906 - t780 + t827;
    let t910 = 1.0_f64 / t279;
    let t911 = sigma0 * t910;
    (t908, t911)
}
