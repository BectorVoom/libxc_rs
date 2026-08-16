//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 953/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk953(t283: f64, t9409: f64, t3201: f64, t982: f64, t1018: f64, t1085: f64, t1017: f64, t86: f64, t2820: f64, t3198: f64) -> (f64, f64, f64, f64) {
    let t9410 = t9409 * t283;
    let t9415 = t3201 * t982;
    let t9423 = t1018 * t1085;
    let t9425 = t86 * t1017 * t9423;
    let t9429 = t86 * t2820 * t3198;
    (t9410, t9415, t9425, t9429)
}
