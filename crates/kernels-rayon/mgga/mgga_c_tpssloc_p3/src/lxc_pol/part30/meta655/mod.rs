//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta655(t7557: f64, t82632: f64, t25836: f64, t3216: f64, t11094: f64, t7627: f64, t28: f64, t40772: f64, t1649: f64, t2752: f64, t26012: f64, t6505: f64, t1437: f64, t6509: f64, t1863: f64, t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64, t3967: f64, t12571: f64, t608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89672, t89698, t89702, t89953, t89992, t90087) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2071(t7557, t82632, t25836, t3216, t11094, t7627, t28, t40772, t1649, t2752, t26012, t6505);
        let (t90091, t90095, t90098, t90101, t90104, t90114) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2072(t1437, t6509, t1863, t1864, t4021, t1410, t9231, t2240, t3961, t3967, t12571, t608);
    (t89672, t89698, t89702, t89953, t89992, t90087, t90091, t90095, t90098, t90101, t90104, t90114)
}
