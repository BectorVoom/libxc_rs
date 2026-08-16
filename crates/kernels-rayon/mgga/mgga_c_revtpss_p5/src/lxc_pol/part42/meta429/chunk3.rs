//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1499/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1499(t108710: f64, t108714: f64, t118749: f64, t1310: f64, t13426: f64, t18245: f64, t1843: f64, t21658: f64, t2198: f64, t2199: f64, t2201: f64, t2322: f64, t30143: f64, t31403: f64, t31451: f64, t31452: f64, t31653: f64, t31663: f64, t31677: f64, t4248: f64, t4254: f64, t508: f64, t5523: f64, t651: f64, t6765: f64, t75439: f64, t7732: f64, t8307: f64, t8320: f64, t8327: f64, t8411: f64, t85360: f64) -> f64 {
    let t118911 = -2.0_f64 * t651 * t21658 * t2198 - 4.0_f64 * t7732 * t31403 - 2.0_f64 * t75439 * t2199 - 2.0_f64 * t85360 * t2199 - 2.0_f64 * t18245 * t8307 + 4.0_f64 * t13426 * t8411 - 2.0_f64 * t651 * t6765 * t8320 - 2.0_f64 * t2322 * t31677 - 2.0_f64 * t4254 * t31677 - 2.0_f64 * t651 * t1310 * t31653 - 4.0_f64 * t4248 * t31452 - 2.0_f64 * t108710 * t2199 - 2.0_f64 * t108714 * t2199 + 2.0_f64 * t5523 * t31663 - 2.0_f64 * t651 * t508 * t118749 + 2.0_f64 * t75439 * t2201 + 2.0_f64 * t85360 * t2201 + 2.0_f64 * t30143 * t8327 - 4.0_f64 * t4248 * t31403 - 4.0_f64 * t651 * t1843 * t31451;
    t118911
}
