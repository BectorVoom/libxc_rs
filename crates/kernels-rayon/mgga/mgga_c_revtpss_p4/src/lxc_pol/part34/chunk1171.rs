//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1171/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1171(t30004: f64, t508: f64, t651: f64, t7898: f64, t7935: f64, t2022: f64, t6895: f64, t25924: f64, t1903: f64, t7910: f64, t7296: f64, t6918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30005 = t508 * t30004;
    let t30007 = 2.0_f64 * t651 * t30005;
    let t30015 = 2.0_f64 * t7898 * t7935;
    let t30016 = t2022 * t6895;
    let t30017 = t25924 * t30016;
    let t30020 = t7910 * t1903;
    let t30021 = t7296 * t30020;
    let t30031 = t2022 * t6918;
    (t30005, t30007, t30015, t30016, t30017, t30020, t30021, t30031)
}
