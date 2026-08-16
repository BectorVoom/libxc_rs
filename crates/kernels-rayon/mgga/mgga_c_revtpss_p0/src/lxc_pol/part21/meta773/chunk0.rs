//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2745/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2745(t50504: f64, t1558: f64, t2722: f64, t10726: f64, t2661: f64, t2724: f64, t4416: f64, t4352: f64, t10722: f64, t4435: f64, t14751: f64, t2652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50505 = 0.22866142996303859718e-3_f64 * t50504;
    let t50511 = t1558 * t2722;
    let t50518 = t2661 * t10726 * t4416 * t2724;
    let t50522 = t2661 * t10726 * t4352 * t2724;
    let t50524 = t10722 * t4435;
    let t50526 = t2652 * t14751;
    (t50505, t50511, t50518, t50522, t50524, t50526)
}
