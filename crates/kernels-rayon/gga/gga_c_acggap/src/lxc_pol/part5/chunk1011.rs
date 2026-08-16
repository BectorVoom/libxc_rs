//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1011/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1011(t5096: f64, t997: f64, t3409: f64, t4364: f64, t3372: f64, t5124: f64, t14223: f64, t5152: f64, t3382: f64, t4335: f64, t4316: f64, t1008: f64, t4932: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17066 = t997 * t5096;
    let t17068 = t3409 * t4364;
    let t17070 = t3372 * t5124;
    let t17072 = t14223 * t5152;
    let t17085 = t3382 * t4335;
    let t17088 = t3409 * t4316;
    let t17090 = t1008 * t4932;
    (t17066, t17068, t17070, t17072, t17085, t17088, t17090)
}
