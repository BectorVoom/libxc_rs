//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 858/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk858(t30083: f64, t177: f64, t377: f64, t7370: f64, t2067: f64, t3077: f64, t7348: f64, t1160: f64, t7432: f64, t7365: f64, t4180: f64, t3427: f64, t7647: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30084 = 0.42874018118069736972e-3_f64 * t30083;
    let t30088 = t377 * t7370 * t177;
    let t30089 = 0.34013387707001991332e-1_f64 * t30088;
    let t30090 = t3077 * t2067;
    let t30091 = t30090 * t7348;
    let t30105 = t1160 * t7432;
    let t30106 = t30105 * t7365;
    let t30120 = t4180 * t2067;
    let t30123 = t7647 * t3427;
    (t30084, t30089, t30090, t30091, t30105, t30106, t30120, t30123)
}
