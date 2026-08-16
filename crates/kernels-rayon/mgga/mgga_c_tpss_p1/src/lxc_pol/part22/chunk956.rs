//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 956/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk956(t493: f64, t9934: f64, t1193: f64, t8115: f64, t8110: f64, t2222: f64, t3190: f64, t1186: f64, t3211: f64, t1170: f64, t3298: f64, t1173: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9936 = 1.0_f64 / t493 / t9934;
    let t9954 = 0.51947577317044391277e2_f64 * t1193 * t8115;
    let t9956 = 0.35089341735807877242e1_f64 * t1193 * t8110;
    let t9957 = t3190 * t2222;
    let t9959 = t3211 * t1186;
    let t9961 = t1170 * t3298;
    let t9963 = t1173 * t3298;
    (t9936, t9954, t9956, t9957, t9959, t9961, t9963)
}
