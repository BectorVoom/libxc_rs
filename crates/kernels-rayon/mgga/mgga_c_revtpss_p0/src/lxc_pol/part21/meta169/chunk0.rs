//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1069/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1069(t124: f64, t3829: f64, t800: f64, t1376: f64, t2689: f64, t1353: f64, t1413: f64, t547: f64, t807: f64, t2700: f64, t535: f64, t1369: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3945 = t124 * t3829;
    let t3946 = t800 * t3945;
    let t3950 = 0.76220476654346199061e-4_f64 * t2689 * t1376;
    let t3951 = t1413 * t1353;
    let t3952 = t547 * t3951;
    let t3953 = t807 * t3952;
    let t3956 = 35.0_f64 / 432.0_f64 * t2700 * t535;
    let t3957 = t794 * t1369;
    (t3945, t3946, t3950, t3951, t3952, t3953, t3956, t3957)
}
