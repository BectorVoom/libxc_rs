//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1259/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1259(t11381: f64, t9061: f64, t1743: f64, t33219: f64, t5703: f64, t11451: f64, t11518: f64, t1690: f64, t11326: f64, t25871: f64, t1030: f64, t25876: f64, t34073: f64) -> (f64, f64, f64, f64, f64) {
    let t34878 = t9061 * t11381;
    let t34881 = t1743 * t33219 * t5703;
    let t34884 = t11518 * t11451 * t1690;
    let t34886 = t11326 * t25871;
    let t34889 = t1030 * t34073 * t25876;
    (t34878, t34881, t34884, t34886, t34889)
}
