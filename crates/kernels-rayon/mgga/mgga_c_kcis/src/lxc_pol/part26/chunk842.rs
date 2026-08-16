//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 842/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk842(t16905: f64, t498: f64, t12147: f64, t5722: f64, t1368: f64, t5705: f64, t3970: f64) -> (f64, f64, f64, f64) {
    let t16906 = t16905 * t498;
    let t16923 = t12147 * t5722;
    let t16925 = t1368 * t16923 / 432.0_f64;
    let t16933 = t12147 * t5705;
    let t16935 = t1368 * t16933 / 432.0_f64;
    let t16937 = t3970 * t498;
    (t16906, t16925, t16935, t16937)
}
