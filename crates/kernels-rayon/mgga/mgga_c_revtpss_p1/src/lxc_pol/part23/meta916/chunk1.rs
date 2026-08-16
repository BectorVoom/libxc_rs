//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2956/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2956(t19082: f64, t4719: f64, t15547: f64, t6219: f64, t6205: f64, t972: f64, t1634: f64, t52877: f64, t6227: f64, t23694: f64, t3011: f64, t4733: f64, t981: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78417 = 0.10526802520742363173e2_f64 * t4719 * t19082;
    let t78422 = 0.35089341735807877242e1_f64 * t15547 * t6219;
    let t78423 = t6205 * t972;
    let t78426 = 0.10526802520742363173e2_f64 * t52877 * t1634 * t78423;
    let t78428 = 0.51947577317044391276e2_f64 * t15547 * t6227;
    let t78429 = t3011 * t23694;
    let t78432 = 0.17315859105681463759e2_f64 * t981 * t78429 * t4733;
    (t78417, t78422, t78423, t78426, t78428, t78432)
}
