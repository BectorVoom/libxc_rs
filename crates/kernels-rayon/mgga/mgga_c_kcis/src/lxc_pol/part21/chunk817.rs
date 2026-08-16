//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 817/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk817(t2895: f64, t9938: f64, t991: f64, t2880: f64, t2904: f64, t24: f64, t2887: f64, t2890: f64, t2877: f64, t984: f64, t2810: f64, t296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9939 = t9938 * t2895;
    let t9940 = t991 * t9939;
    let t9956 = t2880 * t2904;
    let t9957 = t991 * t9956;
    let t9959 = t24 * t2887;
    let t9960 = t9959 * t2890;
    let t9961 = t991 * t9960;
    let t9970 = t984 * t2877;
    let t9985 = 1.0_f64 / t2810 / t296;
    (t9940, t9957, t9959, t9961, t9970, t9985)
}
