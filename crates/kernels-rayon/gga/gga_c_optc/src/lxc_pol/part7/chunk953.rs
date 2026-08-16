//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 953/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk953(t457: f64, t9091: f64, t8936: f64, t914: f64, t1122: f64, t430: f64, t3126: f64, t4356: f64, t4463: f64, t8193: f64, t8914: f64, t438: f64, t8196: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9093 = 0.32196894406625029092e-1_f64 * t457 * t9091;
    let t9094 = t914 * t8936;
    let t9097 = t430 * t1122;
    let t9098 = t4356 * t3126;
    let t9099 = t9097 * t9098;
    let t9102 = t4463 * t8193;
    let t9103 = t430 * t8914;
    let t9104 = t8196 * t438;
    (t9093, t9094, t9097, t9099, t9102, t9103, t9104)
}
