//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 908/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk908(t2326: f64, t900: f64, t9561: f64, t1407: f64, t3178: f64, t3163: f64, t4379: f64, t2293: f64, t2366: f64, t2365: f64, t1429: f64, t6696: f64, t901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9562 = t900 * t2326;
    let t9564 = 0.89376224879626066674e-1_f64 * t9561 * t9562;
    let t9568 = t1407 * t3178;
    let t9569 = 0.38342925953920749676e0_f64 * t9568;
    let t9571 = 0.29792074959875355558e-1_f64 * t4379 * t3163;
    let t9572 = t2366 * t2293;
    let t9573 = t2365 * t9572;
    let t9575 = 0.29792074959875355558e-1_f64 * t1429 * t9573;
    let t9577 = 0.29792074959875355558e-1_f64 * t6696 * t901;
    (t9562, t9564, t9569, t9571, t9572, t9573, t9575, t9577)
}
