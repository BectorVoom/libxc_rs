//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1290/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1290(t1983: f64, t93982: f64, t1096: f64, t4982: f64, t1043: f64, t1976: f64, t3153: f64, t1981: f64, t42058: f64, t7143: f64, t1982: f64, t93484: f64) -> (f64, f64, f64, f64, f64) {
    let t93983 = t1983 * t93982;
    let t93984 = t4982 * t1096;
    let t93988 = t1976 * t1043;
    let t93989 = t93988 * t3153;
    let t93994 = t1981 * t42058 * t7143;
    let t94005 = t1982 * t93484;
    (t93983, t93984, t93989, t93994, t94005)
}
