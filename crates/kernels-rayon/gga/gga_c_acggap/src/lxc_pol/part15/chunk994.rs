//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 994/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk994(t7447: f64, t8813: f64, t8817: f64, t7440: f64, t8820: f64, t2274: f64, t30307: f64, t1181: f64, t23688: f64, t599: f64, t7346: f64, t7433: f64, t8966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35070 = t7447 * t8813;
    let t35072 = t7447 * t8817;
    let t35074 = t7440 * t8820;
    let t35076 = t30307 * t2274;
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35090 = t7433 * t8966;
    (t35070, t35072, t35074, t35076, t35088, t35090)
}
