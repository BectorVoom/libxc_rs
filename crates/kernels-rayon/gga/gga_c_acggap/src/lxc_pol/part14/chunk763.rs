//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 763/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk763(t1095: f64, t1479: f64, t7476: f64, t1980: f64, t1988: f64, t2304: f64, t1089: f64, t2302: f64, t3201: f64, t598: f64, t137: f64, t1487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8555 = t7476 * t1095 * t1479;
    let t8556 = t1980 * t8555;
    let t8558 = t1988 * t2304;
    let t8561 = t1089 * t3201 * t2302;
    let t8562 = t598 * t8561;
    let t8564 = t137 * t1487;
    (t8555, t8556, t8558, t8561, t8562, t8564)
}
