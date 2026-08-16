//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 693/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk693(t13437: f64, t1445: f64, t1562: f64, t3377: f64, t3566: f64, t11362: f64, t12969: f64, t13397: f64, t912: f64, t587: f64, t6915: f64, t6914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13438 = t1445 * t13437;
    let t13440 = 0.69017266717057349418e1_f64 * t1562 * t13438;
    let t13442 = 0.25025342966295298669e1_f64 * t3566 * t3377;
    let t13444 = 0.10725146985555128001e1_f64 * t11362 * t3377;
    let t13463 = 0.17875244975925213335e0_f64 * t12969;
    let t13465 = t912 * t13397;
    let t13466 = t587 * t13465;
    let t13468 = t6915 * t13397;
    let t13469 = t6914 * t13468;
    (t13438, t13440, t13442, t13444, t13463, t13465, t13466, t13468, t13469)
}
