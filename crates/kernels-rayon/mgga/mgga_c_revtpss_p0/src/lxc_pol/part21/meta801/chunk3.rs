//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2910/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2910(t291: f64, t52782: f64, t52803: f64, t11531: f64, t15421: f64, t2942: f64, t4644: f64, t11408: f64, t1614: f64, t11411: f64, t11502: f64, t11510: f64, t15343: f64, t1634: f64, t2945: f64, t3007: f64, t3015: f64, t41746: f64, t4685: f64, t52522: f64, t52536: f64, t52549: f64, t52562: f64, t52574: f64, t52588: f64, t52601: f64, t52615: f64, t52628: f64, t52637: f64, t52642: f64, t52647: f64, t52650: f64, t52652: f64, t52762: f64, t946: f64, t954: f64, t974: f64) -> (f64, f64, f64) {
    let t52806 = 0.621814e-1_f64 * (t52782 + t52803) * t291;
    let t52808 = 6.0_f64 * t15421 * t11531;
    let t52809 = t4644 * t2942;
    let t52812 = t1614 * t11408;
    let t52817 = 0.17544670867903938621e1_f64 * t52522 * t974 + 1.0_f64 * t946 * (t52536 + t52549 + t52562 + t52574 + t52588 + t52601 + t52615 + t52628) * t954 + 0.17544670867903938621e1_f64 * t15343 * t3007 + 0.51947577317044391276e2_f64 * t52637 * t3015 + 0.5848223622634646207e0_f64 * t4685 * t11502 + 0.10254018858216406658e4_f64 * t52642 * t11510 - t52647 - t52650 - t52652 - t52762 + t52806 - t52808 - 6.0_f64 * t52809 * t2945 - 0.19298375398431042081e3_f64 * t52812 * t11411 + 0.5848223622634646207e0_f64 * t41746 * t1634;
    (t52806, t52808, t52817)
}
