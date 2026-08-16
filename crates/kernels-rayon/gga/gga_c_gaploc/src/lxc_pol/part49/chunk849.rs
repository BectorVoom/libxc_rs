//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 849/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk849(t10241: f64, t9448: f64, t9439: f64, t1339: f64, t3394: f64, t2754: f64, t4130: f64, t20550: f64, t1: f64, t31740: f64, t544: f64, t2875: f64, t6514: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34814 = t9448 * t10241;
    let t34818 = t9439 * t10241;
    let t34890 = t1339 * t3394;
    let t35091 = t4130 * t2754;
    let t35101 = t20550 * t10241;
    let t35106 = t544 * t31740 * t1;
    let t35180 = t544 * t6514 * t2875;
    (t34814, t34818, t34890, t35091, t35101, t35106, t35180)
}
