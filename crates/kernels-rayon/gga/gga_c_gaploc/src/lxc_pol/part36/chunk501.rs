//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 501/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk501(t1570: f64, t3085: f64, t1339: f64, t475: f64, t3158: f64, t494: f64, t3116: f64, t555: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9181 = t1570 * t3085;
    let t9182 = t1339 * t475;
    let t9183 = t9181 * t9182;
    let t9186 = t3158 * t494;
    let t9189 = t555 * t3116;
    let t9190 = t9189 * t494;
    let t9193 = t599 * t3085;
    (t9181, t9182, t9183, t9186, t9189, t9190, t9193)
}
