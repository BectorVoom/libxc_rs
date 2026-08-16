//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 580/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk580(t2197: f64, t3492: f64, t10713: f64, t1445: f64, t833: f64, t10717: f64, t1022: f64, t5241: f64, t2679: f64, t9805: f64, t1029: f64, t9796: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11043 = 0.11502877786176224903e2_f64 * t2197 * t3492;
    let t11044 = t1445 * t10713;
    let t11046 = 0.11502877786176224903e2_f64 * t833 * t11044;
    let t11047 = t1445 * t10717;
    let t11049 = 0.11502877786176224903e2_f64 * t833 * t11047;
    let t11053 = t5241 * t1022;
    let t11054 = t11053 * t2679;
    let t11055 = t9805 * t11054;
    let t11056 = 0.57514388930881124514e0_f64 * t11055;
    let t11057 = t1029 * t2679;
    let t11058 = t9796 * t11057;
    (t11043, t11046, t11049, t11053, t11055, t11056, t11058)
}
