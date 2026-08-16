//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 789/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk789(t40088: f64, t4782: f64, t9272: f64, t20700: f64, t6710: f64, t9438: f64, t12535: f64, t1407: f64, t20551: f64, t6914: f64, t12531: f64, t587: f64, t589: f64) -> (f64, f64, f64, f64, f64) {
    let t40353 = t9272 * t4782 * t40088;
    let t40372 = t6710 * t9438 * t20700;
    let t40374 = t1407 * t12535;
    let t40377 = t6914 * t9438 * t20551;
    let t40380 = t587 * t589 * t12531;
    (t40353, t40372, t40374, t40377, t40380)
}
