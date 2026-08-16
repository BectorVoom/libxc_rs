//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2674/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2674(t11922: f64, t15906: f64, t19753: f64, t20090: f64, t3115: f64, t19649: f64, t372: f64, t11774: f64, t20039: f64, t53405: f64, t19837: f64, t19744: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66288 = t15906 * t11922 * t19753;
    let t66304 = t3115 * t11922 * t20090;
    let t66306 = t372 * t19649;
    let t66328 = t11774 * t53405 * t20039;
    let t66332 = t3115 * t11922 * t19837;
    let t66355 = t3115 * t11922 * t19744;
    (t66288, t66304, t66306, t66328, t66332, t66355)
}
