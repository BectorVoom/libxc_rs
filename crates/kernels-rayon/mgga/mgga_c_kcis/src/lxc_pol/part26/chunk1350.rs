//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1350/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1350(t28351: f64, t75638: f64, t28335: f64, t28392: f64, t16823: f64, t5737: f64, t1307: f64, t21827: f64, t5709: f64, t21868: f64, t491: f64, t990: f64) -> (f64, f64, f64, f64, f64) {
    let t103063 = t28351 * t75638;
    let t103066 = t28392 * t28335;
    let t103069 = t28351 * t16823 * t5737;
    let t103073 = t5709 * t21827 * t1307;
    let t103078 = t21868 * t491 * t990;
    (t103063, t103066, t103069, t103073, t103078)
}
