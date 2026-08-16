//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1145/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1145(t1538: f64, t30802: f64, t9267: f64, t4782: f64, t9272: f64, t21272: f64, t9544: f64, t2349: f64, t2478: f64, t6576: f64, t7047: f64, t888: f64) -> (f64, f64, f64, f64, f64) {
    let t30805 = 0.38342925953920749676e1_f64 * t9267 * t1538 * t30802;
    let t30808 = 0.23005755572352449806e1_f64 * t9272 * t4782 * t30802;
    let t30809 = t21272 * t9544;
    let t30812 = t6576 * t2349 * t2478;
    let t30820 = t6576 * t888 * t7047;
    (t30805, t30808, t30809, t30812, t30820)
}
