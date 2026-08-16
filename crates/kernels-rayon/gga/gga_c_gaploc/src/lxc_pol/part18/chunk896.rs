//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 896/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk896(t3122: f64, t484: f64, t3130: f64, t123: f64, t6514: f64, t2326: f64, t9074: f64, t4261: f64, t6510: f64, t584: f64, t6575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9149 = 0.31616674039640166221e-2_f64 * t484 * t3122;
    let t9151 = 0.31616674039640166221e-2_f64 * t484 * t3130;
    let t9204 = t6514 * t123;
    let t9205 = t9204 * t2326;
    let t9207 = 0.71137516589190373998e-2_f64 * t9074 * t9205;
    let t9208 = t4261 * t6510;
    let t9210 = 0.47425011059460249332e-2_f64 * t9074 * t9208;
    let t9263 = t584 * t6575;
    (t9149, t9151, t9204, t9205, t9207, t9208, t9210, t9263)
}
