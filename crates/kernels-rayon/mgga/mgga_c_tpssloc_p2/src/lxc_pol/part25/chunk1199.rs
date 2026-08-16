//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1199/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1199(t225: f64, t24162: f64, t81317: f64, t12030: f64, t12437: f64, t1375: f64, t1386: f64, t2091: f64, t2092: f64, t24082: f64, t3887: f64, t3911: f64, t3912: f64, t39910: f64, t7199: f64, t7213: f64, t81307: f64, t81311: f64, t81315: f64, t81328: f64) -> f64 {
    let t84655 = t24162 * t225;
    let t84659 = 0.55440370401180965083e0_f64 * t81317;
    let t84667 = 2.0_f64 * t1375 * t3887 * t2091 * t12437 + 6.0_f64 * t12030 * t7199 - 0.11514538467937585055e0_f64 * t81307 - 0.49348022005446793095e-1_f64 * t81311 - t39910 * t2092 - 3.0_f64 * t84655 * t1386 + 0.9869604401089358619e-1_f64 * t81315 - t84659 - 0.9869604401089358619e-1_f64 * t81328 + 6.0_f64 * t1375 * t3887 * t7213 * t3911 - 3.0_f64 * t24082 * t3912;
    t84667
}
