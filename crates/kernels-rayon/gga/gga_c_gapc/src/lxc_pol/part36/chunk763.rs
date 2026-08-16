//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 763/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk763(t144: f64, t3694: f64, t3116: f64, t9090: f64, t3028: f64, t677: f64, t3021: f64, t5075: f64, t1043: f64, t5979: f64, t8820: f64, t5977: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9091 = t3694 * t144;
    let t9092 = t9091 * t3116;
    let t9093 = t9090 * t9092;
    let t9097 = t3028 * t677;
    let t9099 = t3021 * t5075;
    let t9100 = t1043 * t9099;
    let t9103 = t8820 * t144 * t5979;
    let t9104 = t5977 * t9103;
    (t9092, t9093, t9097, t9099, t9100, t9104)
}
