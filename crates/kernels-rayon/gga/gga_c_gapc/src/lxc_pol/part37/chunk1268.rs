//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1268/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1268(t11540: f64, t424: f64, t612: f64, t11333: f64, t5252: f64, t1386: f64, t3157: f64, t3674: f64, t11561: f64, t8734: f64, t116: f64, t25110: f64, t27145: f64, t33781: f64) -> (f64, f64, f64, f64, f64) {
    let t35034 = t424 * t612 * t11540;
    let t35036 = t5252 * t11333;
    let t35039 = t1386 * t3674 * t3157;
    let t35041 = t11561 * t8734;
    let t35045 = t116 * t33781 * t25110 * t27145;
    (t35034, t35036, t35039, t35041, t35045)
}
