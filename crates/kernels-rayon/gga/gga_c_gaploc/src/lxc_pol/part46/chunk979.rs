//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 979/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk979(t1445: f64, t43217: f64, t833: f64, t43316: f64, t13136: f64, t2197: f64, t10040: f64, t25198: f64, t41133: f64, t11112: f64, t2679: f64, t9800: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43640 = 0.11502877786176224903e2_f64 * t833 * t1445 * t43217;
    let t43642 = t833 * t1445 * t43316;
    let t43645 = 0.11502877786176224903e2_f64 * t2197 * t13136;
    let t43646 = t25198 * t10040;
    let t43647 = 0.89376224879626066675e-1_f64 * t43646;
    let t43648 = 0.19171462976960374838e1_f64 * t41133;
    let t43650 = t9800 * t11112 * t2679;
    (t43640, t43642, t43645, t43647, t43648, t43650)
}
