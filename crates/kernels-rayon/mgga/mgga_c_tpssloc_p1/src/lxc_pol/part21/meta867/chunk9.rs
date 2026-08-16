//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3173/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3173(t15486: f64, t5005: f64, t1222: f64, t18574: f64, t1174: f64, t15527: f64, t1748: f64, t19033: f64, t3440: f64, t3527: f64, t3531: f64, t3587: f64, t5019: f64, t53487: f64, t63390: f64, t65660: f64, t65662: f64, t65664: f64, t65668: f64, t65670: f64, t65672: f64, t65674: f64) -> f64 {
    let t65676 = t5005 * t15486;
    let t65681 = t18574 * t1222;
    let t65685 = -19.0_f64 / 2592.0_f64 * t19033 * t3527 - 19.0_f64 / 1296.0_f64 * t19033 * t3531 - t5019 * t15527 / 288.0_f64 + t65660 / 1152.0_f64 + 5.0_f64 / 10368.0_f64 * t65662 - 19.0_f64 / 7776.0_f64 * t65664 - t53487 * t1748 / 2304.0_f64 + t65668 / 324.0_f64 + 19.0_f64 / 1296.0_f64 * t65670 - 19.0_f64 / 1944.0_f64 * t65672 - t65674 / 2304.0_f64 - t65676 / 1728.0_f64 + t1174 * t3440 * t63390 / 6.0_f64 + t65681 / 2304.0_f64 + 95.0_f64 / 7776.0_f64 * t19033 * t3587;
    t65685
}
