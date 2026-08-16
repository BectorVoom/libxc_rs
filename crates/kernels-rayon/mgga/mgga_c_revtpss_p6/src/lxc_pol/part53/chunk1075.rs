//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1075/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1075(t1937: f64, t34446: f64, t7586: f64, t7735: f64, t1936: f64, t29427: f64, t7741: f64, t1518: f64, t32825: f64, t33633: f64, t33635: f64, t33637: f64, t33640: f64, t33642: f64, t33644: f64, t33646: f64, t34419: f64, t8564: f64) -> (f64, f64, f64) {
    let t34447 = t34446 * t1937;
    let t34449 = t7586 * t7735;
    let t34453 = t29427 * t1936;
    let t34455 = t34446 * t1936;
    let t34457 = t7586 * t7741;
    let t34462 = 2.0_f64 * t1518 * t32825 + 2.0_f64 * t33633 + 2.0_f64 * t33635 + 2.0_f64 * t33637 + t33640 + t33642 + t33644 + t33646 + t34419 + 2.0_f64 * t34453 + 2.0_f64 * t34455 + 2.0_f64 * t34457 + t8564;
    (t34447, t34449, t34462)
}
