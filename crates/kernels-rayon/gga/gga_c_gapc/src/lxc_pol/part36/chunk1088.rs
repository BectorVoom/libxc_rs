//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1088/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1088(t11302: f64, t15811: f64, t18824: f64, t33373: f64, t7259: f64, t8142: f64, t11320: f64, t2629: f64, t933: f64, t11597: f64, t3408: f64, t9563: f64) -> (f64, f64, f64, f64) {
    let t33380 = t15811 * t11302 * t18824;
    let t33383 = t7259 * t33373 * t8142;
    let t33387 = t933 * t11320 * t2629;
    let t33390 = t9563 * t11597 * t3408;
    (t33380, t33383, t33387, t33390)
}
