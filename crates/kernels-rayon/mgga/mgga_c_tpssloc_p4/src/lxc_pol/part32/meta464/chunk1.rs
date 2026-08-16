//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1752/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1752(t24574: f64, t7303: f64, t7291: f64, t2123: f64, t3427: f64, t2121: f64, t221: f64, t3448: f64, t2127: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24575 = t24574 * t7303;
    let t24577 = t24574 * t7291;
    let t24585 = t3427 * t2123;
    let t24587 = 0.18277045187202515961e-2_f64 * t2121 * t24585;
    let t24588 = t221 * t3448;
    let t24589 = t2127 * t24588;
    (t24575, t24577, t24585, t24587, t24588, t24589)
}
