//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 981/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk981(t13055: f64, t5640: f64, t13058: f64, t1991: f64, t20671: f64, t28309: f64, t33601: f64, t13023: f64, t4614: f64, t833: f64, t43008: f64, t4820: f64, t7513: f64) -> (f64, f64, f64, f64, f64) {
    let t43652 = t5640 * t13055;
    let t43653 = 0.15337170381568299871e1_f64 * t43652;
    let t43657 = t1991 * t13058;
    let t43658 = 0.1022478025437886658e1_f64 * t43657;
    let t43660 = t28309 * t20671 * t33601;
    let t43661 = 0.17041300423964777634e0_f64 * t43660;
    let t43664 = 0.15337170381568299871e2_f64 * t833 * t4614 * t13023;
    let t43666 = t7513 * t4820 * t43008;
    (t43653, t43658, t43661, t43664, t43666)
}
