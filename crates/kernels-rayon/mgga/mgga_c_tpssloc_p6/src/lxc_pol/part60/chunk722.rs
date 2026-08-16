//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 722/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk722(t2123: f64, t3427: f64, t2121: f64, t221: f64, t3448: f64, t2127: f64, t3439: f64, t461: f64, t491: f64, t225: f64, t1089: f64, t1240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24585 = t3427 * t2123;
    let t24587 = 0.18277045187202515961e-2_f64 * t2121 * t24585;
    let t24588 = t221 * t3448;
    let t24589 = t2127 * t24588;
    let t24594 = t3439 * t461;
    let t24595 = t24594 * t491;
    let t24600 = t461 * t491;
    let t24601 = t24600 * t225;
    let t24602 = t1240 * t1089;
    (t24587, t24589, t24594, t24595, t24601, t24602)
}
