//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2180/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2180(t11153: f64, t460: f64, t9288: f64, t3242: f64, t405: f64, t974: f64, t11509: f64, t1174: f64, t15281: f64, t11525: f64, t3431: f64, t1176: f64, t2402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44607 = t460 * t11153;
    let t44608 = t44607 * t9288;
    let t44620 = 1.0_f64 / t405 / t3242;
    let t44621 = t974 * t44620;
    let t44628 = t1174 * t15281 * t11509;
    let t44631 = t1174 * t3431 * t11525;
    let t44633 = t2402 * t1176;
    (t44607, t44608, t44620, t44621, t44628, t44631, t44633)
}
