//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 954/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk954(t3340: f64, t995: f64, t3356: f64, t3323: f64, t10415: f64, t10418: f64, t10423: f64, t10438: f64, t10441: f64, t10445: f64, t28: f64, t3330: f64, t3334: f64, t3347: f64, t34: f64, t38: f64, t984: f64, t991: f64, tau1: f64) -> (f64, f64, f64, f64) {
    let t10451 = t3340 * t995;
    let t10454 = t995 * t3356;
    let t10463 = tau1 * t3323;
    let t10478 = -10.0_f64 / 27.0_f64 * t34 * t10415 + 10.0_f64 / 3.0_f64 * t34 * t10418 + 5.0_f64 / 3.0_f64 * t34 * t10423 - 440.0_f64 / 27.0_f64 * t10463 * t28 + 200.0_f64 / 9.0_f64 * t3347 * t984 - 50.0_f64 / 9.0_f64 * t991 * t3330 - 25.0_f64 / 3.0_f64 * t991 * t3334 - 10.0_f64 / 27.0_f64 * t38 * t10438 + 10.0_f64 / 3.0_f64 * t38 * t10441 + 5.0_f64 / 3.0_f64 * t38 * t10445;
    (t10451, t10454, t10463, t10478)
}
