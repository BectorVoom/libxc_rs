//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1165/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1165(t28658: f64, t440: f64, t1430: f64, t3318: f64, t2489: f64, t8635: f64, t10422: f64, t1424: f64, t16129: f64, t82: f64, t15: f64, t10415: f64, t10418: f64, t10423: f64, t10463: f64, t19523: f64, t2500: f64, t28649: f64, t28653: f64, t3347: f64, t34: f64, t445: f64, t454: f64, t6723: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28659 = t28658 * t440;
    let t28662 = t1430 * t3318;
    let t28665 = t2489 * t8635;
    let t28671 = t1424 * t10422 * t440;
    let t28676 = 6.0_f64 * t82 + 12.0_f64 * t16129;
    let t28677 = t15 * t28676;
    let t28684 = 50.0_f64 / 81.0_f64 * t454 * t10415 + 40.0_f64 / 81.0_f64 * t34 * t28649 - 10.0_f64 / 9.0_f64 * t19523 * t28653 - 50.0_f64 / 9.0_f64 * t454 * t10418 - 10.0_f64 / 9.0_f64 * t19523 * t28659 + 10.0_f64 / 3.0_f64 * t6723 * t28662 + 10.0_f64 / 3.0_f64 * t34 * t28665 - 25.0_f64 / 9.0_f64 * t454 * t10423 + 10.0_f64 / 9.0_f64 * t34 * t28671 + 5.0_f64 / 3.0_f64 * t34 * t28677 - 2200.0_f64 / 81.0_f64 * t10463 * t445 + 400.0_f64 / 27.0_f64 * t3347 * t2500;
    (t28659, t28662, t28665, t28671, t28676, t28677, t28684)
}
