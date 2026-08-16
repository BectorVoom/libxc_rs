//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1306/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1306(t26983: f64, t7635: f64, t1210: f64, t29193: f64, t2142: f64, t3153: f64, t3601: f64, t1203: f64, t5464: f64, t26894: f64, t3588: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96966 = t26983 * t7635;
    let t96979 = t1210 * t29193;
    let t96981 = t2142 * t3601 * t3153;
    let t96982 = t5464 * t1203;
    let t96986 = t26894 * t29193;
    let t97010 = t2142 * t3588;
    let t97011 = t97010 * t73;
    (t96966, t96979, t96981, t96982, t96986, t97010, t97011)
}
