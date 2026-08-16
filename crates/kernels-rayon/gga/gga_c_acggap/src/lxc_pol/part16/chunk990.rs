//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 990/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk990(t7447: f64, t8813: f64, t8817: f64, t7440: f64, t8820: f64, t2274: f64, t30307: f64, t1181: f64, t23688: f64, t599: f64, t7346: f64, t7433: f64, t8966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35070 = t7447 * t8813;
    let t35071 = 0.84046875e-1_f64 * t35070;
    let t35072 = t7447 * t8817;
    let t35073 = 0.84046875e-1_f64 * t35072;
    let t35074 = t7440 * t8820;
    let t35075 = 0.5603125e-1_f64 * t35074;
    let t35076 = t30307 * t2274;
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35089 = 0.21437009059034868486e-3_f64 * t35088;
    let t35090 = t7433 * t8966;
    (t35071, t35073, t35075, t35076, t35089, t35090)
}
