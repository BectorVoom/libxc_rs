//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1002/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1002(t174: f64, t740: f64, t9323: f64, t447: f64, t2001: f64, t4134: f64, t1610: f64, t2104: f64, t2153: f64, t2539: f64, t9275: f64, t2146: f64, t2537: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t18374 = 2.0_f64 * t740;
    let t18375 = 6.0_f64 * t9323;
    let t18376 = -t18374 + t18375;
    let t18377 = piecewise3(t175, 0.0_f64, t18376);
    let t18378 = t447 * t18377;
    let t20905 = t4134 * t2001;
    let t23096 = t2104 * t1610;
    let t26390 = t2153 * t2539;
    let t26391 = t9275 * t26390;
    let t26392 = 6.0_f64 * t26391;
    let t26398 = t2146 * t2537;
    (t18377, t18378, t20905, t23096, t26390, t26391, t26392, t26398)
}
