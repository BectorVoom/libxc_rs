//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 918/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk918(t498: f64, t9904: f64, t1170: f64, t3197: f64, t1186: f64, t3214: f64, t30: f64, t490: f64, t33: f64, t493: f64, t1193: f64, t8115: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9906 = 120.0_f64 * t9904 * t498;
    let t9907 = t1170 * t3197;
    let t9913 = t3214 * t1186;
    let t9922 = t30 * t30;
    let t9924 = 1.0_f64 / t490 / t9922;
    let t9934 = t33 * t33;
    let t9936 = 1.0_f64 / t493 / t9934;
    let t9954 = 0.51947577317044391277e2_f64 * t1193 * t8115;
    (t9906, t9907, t9913, t9924, t9936, t9954)
}
