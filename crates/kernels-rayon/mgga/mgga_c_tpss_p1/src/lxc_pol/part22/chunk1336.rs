//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1336/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1336(t12853: f64, t5728: f64, t18450: f64, t4462: f64, t12960: f64, t5721: f64, t12869: f64, t19476: f64, t4473: f64, t60738: f64, t12873: f64, t18454: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65626 = t5728 * t12853;
    let t65628 = t18450 * t4462;
    let t65630 = t5721 * t12960;
    let t65636 = t19476 * t12869;
    let t65639 = t60738 * t4473;
    let t65641 = t18454 * t12873;
    (t65626, t65628, t65630, t65636, t65639, t65641)
}
