//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2058/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2058(t26884: f64, t460: f64, t1210: f64, t26921: f64, t3627: f64, t5457: f64, t26983: f64, t7635: f64, t29193: f64, t26894: f64, t3566: f64, t7627: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96938 = t460 * t26884;
    let t96953 = t1210 * t26921;
    let t96954 = t3627 * t5457;
    let t96966 = t26983 * t7635;
    let t96979 = t1210 * t29193;
    let t96986 = t26894 * t29193;
    let t97019 = t3566 * t7627;
    (t96938, t96953, t96954, t96966, t96979, t96986, t97019)
}
