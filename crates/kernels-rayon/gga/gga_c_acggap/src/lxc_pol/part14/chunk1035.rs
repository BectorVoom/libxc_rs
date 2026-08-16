//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1035/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1035(t36388: f64, t1967: f64, t8566: f64, t1998: f64, t4557: f64, t5351: f64, t7948: f64, t309: f64, t556: f64, t322: f64, t29979: f64, t620: f64) -> (f64, f64, f64, f64, f64) {
    let t36389 = 0.34299214494455789578e-2_f64 * t36388;
    let t36390 = t1967 * t8566;
    let t36391 = 0.37737710747524982482e-2_f64 * t36390;
    let t36392 = t1998 * t4557;
    let t36405 = t7948 * t5351;
    let t36416 = t556 * t309;
    let t36417 = t36416 * t322;
    let t36419 = t29979 * t620 * t36417;
    (t36389, t36391, t36392, t36405, t36419)
}
