//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 524/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk524(t3152: f64, t448: f64, t3148: f64, t2295: f64, t894: f64, t3116: f64, t599: f64, t475: f64, t2343: f64, t1564: f64, t3085: f64, t6320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9162 = t3152 * t448;
    let t9165 = t3148 * t448;
    let t9168 = t894 * t2295;
    let t9171 = t599 * t3116;
    let t9172 = t9171 * t475;
    let t9173 = t2343 * t9172;
    let t9176 = t1564 * t3085;
    let t9177 = t9176 * t475;
    let t9178 = t6320 * t9177;
    (t9162, t9165, t9168, t9171, t9172, t9173, t9176, t9177, t9178)
}
