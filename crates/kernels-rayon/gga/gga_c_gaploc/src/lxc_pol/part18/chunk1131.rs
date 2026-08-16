//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1131/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1131(t1358: f64, t9208: f64, t1365: f64, t20692: f64, t6525: f64, t1349: f64, t9083: f64, t2317: f64, t6289: f64, t1217: f64, t3122: f64, t1222: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30145 = 0.12646669615856066488e-1_f64 * t1358 * t9208;
    let t30148 = 0.47425011059460249332e-2_f64 * t6525 * t1365 * t20692;
    let t30152 = 0.63233348079280332442e-2_f64 * t1349 * t9083;
    let t30169 = 0.47425011059460249332e-2_f64 * t6525 * t6289 * t2317;
    let t30171 = 0.73772239425827054516e-2_f64 * t1217 * t3122;
    let t30173 = 0.63233348079280332442e-2_f64 * t1222 * t3122;
    (t30145, t30148, t30152, t30169, t30171, t30173)
}
