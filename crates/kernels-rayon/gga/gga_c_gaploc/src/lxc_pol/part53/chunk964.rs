//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 964/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk964(t2617: f64, t3726: f64, t7803: f64, t2679: f64, t9796: f64, t12240: f64, t7810: f64, t38961: f64, t935: f64, t1457: f64, t2103: f64, t12213: f64, t2530: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47206 = t7803 * t3726 * t2617;
    let t47212 = t9796 * t3726 * t2679;
    let t47215 = t7810 * t12240 * t2617;
    let t47220 = t38961 * t935;
    let t47222 = t2103 * t1457 * t47220;
    let t47225 = t12213 * t2530;
    (t47206, t47212, t47215, t47220, t47222, t47225)
}
