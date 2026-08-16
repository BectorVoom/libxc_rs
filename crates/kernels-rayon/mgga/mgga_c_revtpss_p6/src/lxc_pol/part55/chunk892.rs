//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 892/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk892(t1937: f64, t27126: f64, t6993: f64, t7732: f64, t7003: f64, t2322: f64, t7735: f64, t4254: f64, t1936: f64, t5517: f64, t651: f64, t7235: f64, t7935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27128 = 2.0_f64 * t27126 * t1937;
    let t27130 = 2.0_f64 * t7732 * t6993;
    let t27132 = 2.0_f64 * t7732 * t7003;
    let t27134 = 2.0_f64 * t2322 * t7735;
    let t27136 = 2.0_f64 * t4254 * t7735;
    let t27137 = t5517 * t1936;
    let t27139 = 2.0_f64 * t651 * t27137;
    let t27152 = t7235 * t7935;
    (t27128, t27130, t27132, t27134, t27136, t27137, t27139, t27152)
}
