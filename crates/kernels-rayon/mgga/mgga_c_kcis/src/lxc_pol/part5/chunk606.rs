//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 606/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk606(t468: f64, t3793: f64, t1341: f64, t45: f64, t1346: f64, t478: f64) -> (f64, f64, f64, f64, f64) {
    let t3900 = t468 * t468;
    let t3901 = 1.0_f64 / t3900;
    let t3905 = 0.12361111111111111111e-1_f64 * t3793;
    let t3914 = t45 * t1341;
    let t3917 = t1346 * t478;
    let t3918 = 1.0_f64 / t3917;
    (t3900, t3901, t3905, t3914, t3918)
}
