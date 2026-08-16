//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 645/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk645(t7049: f64, t7266: f64, t589: f64, t2069: f64, t5897: f64) -> (f64, f64, f64, f64) {
    let t7267 = t7049 + t7266;
    let t7268 = t7267 * t589;
    let t7270 = 2.0_f64 * t5897 * t2069;
    let t7271 = t2069 * t2069;
    (t7267, t7268, t7270, t7271)
}
