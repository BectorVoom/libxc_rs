//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1196/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1196(t1913: f64, t8734: f64, t34369: f64, t571: f64, t102005: f64, t28196: f64, t34297: f64, t670: f64, t8626: f64, t122570: f64, t125496: f64, t1519: f64, t2055: f64, t2322: f64, t27830: f64, t28929: f64, t32389: f64, t32621: f64, t33578: f64, t33580: f64, t33583: f64, t34290: f64, t4254: f64, t4257: f64, t4293: f64, t651: f64, t7732: f64) -> (f64, f64, f64, f64) {
    let t127515 = t1913 * t8734;
    let t127516 = t571 * t34369;
    let t127532 = 2.0_f64 * t28196 * t102005 * t34297;
    let t127535 = t8626 * t670;
    let t127540 = -2.0_f64 * t2055 * t27830 * t651 - 2.0_f64 * t122570 * t1519 + 6.0_f64 * t125496 * t28929 - 2.0_f64 * t127535 * t1519 - 2.0_f64 * t2322 * t34290 - 2.0_f64 * t32389 * t4257 - 2.0_f64 * t32389 * t4293 - 2.0_f64 * t32621 * t7732 - 2.0_f64 * t34290 * t4254 + t127532 - t33578 - t33580 - t33583;
    (t127515, t127516, t127535, t127540)
}
