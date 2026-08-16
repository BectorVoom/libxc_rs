//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 882/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk882(t167: f64, t1773: f64, t3211: f64, t3210: f64, t13172: f64, t13192: f64, t4802: f64, t4793: f64, t4797: f64, t1121: f64, t6272: f64, t4555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19148 = t167 * t1773;
    let t19149 = t3211 * t19148;
    let t19150 = t3210 * t19149;
    let t19151 = t13172 * t19150;
    let t19153 = t13192 * t4802;
    let t19155 = t13192 * t4793;
    let t19157 = t13192 * t4797;
    let t19159 = t6272 * t1121;
    let t19160 = t4555 * t19159;
    (t19149, t19151, t19153, t19155, t19157, t19159, t19160)
}
