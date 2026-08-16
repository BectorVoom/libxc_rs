//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1337/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1337(t1791: f64, t69198: f64, t69206: f64, t19349: f64, t67472: f64, t1792: f64, t18350: f64, t18666: f64, t18673: f64, t19342: f64, t21136: f64, t25752: f64, t5794: f64, t62294: f64, t67378: f64, t67385: f64, t69087: f64, t69195: f64, t69203: f64, t69210: f64, t7690: f64) -> f64 {
    let t71396 = t1791 * t69198;
    let t71401 = t1791 * t69206;
    let t71404 = t19349 * t67472;
    let t71411 = -40.0_f64 * t7690 * t25752 * t19342 - 2.0_f64 / 3.0_f64 * t69087 * t1792 - 2.0_f64 / 3.0_f64 * t21136 * t5794 + 20.0_f64 * t18666 * t69195 + 20.0_f64 / 3.0_f64 * t18350 * t71396 + 10.0_f64 * t18666 * t69203 + 10.0_f64 / 3.0_f64 * t18350 * t71401 - 160.0_f64 / 9.0_f64 * t71404 + 10.0_f64 / 3.0_f64 * t69210 * t18673 + 20.0_f64 / 3.0_f64 * t19349 * t67378 - t62294 + 176.0_f64 / 27.0_f64 * t67385;
    t71411
}
