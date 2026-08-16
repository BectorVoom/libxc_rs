//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1010/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1010(t12043: f64, t12073: f64, t12085: f64, t12086: f64, t12106: f64, t12131: f64, t12133: f64, t12145: f64, t502: f64, t3751: f64, t617: f64, t1628: f64, t3745: f64) -> (f64, f64, f64, f64) {
    let t12148 = t12043 + t12073 + t12085 + t12086 + t12106 + t12131 + t12133 + t12145;
    let t12149 = t502 * t12148;
    let t12150 = t617 * t3751;
    let t12153 = t1628 * t3745;
    (t12148, t12149, t12150, t12153)
}
