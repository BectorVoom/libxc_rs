//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 518/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk518(t3153: f64, t3159: f64, t1160: f64, t1172: f64, t360: f64, t435: f64) -> (f64, f64, f64, f64) {
    let t3160 = t3153 * t3159;
    let t3161 = 5.0_f64 / 6.0_f64 * t3160;
    let t3194 = t1160 * t1172;
    let t3201 = t435 * t360;
    (t3160, t3161, t3194, t3201)
}
