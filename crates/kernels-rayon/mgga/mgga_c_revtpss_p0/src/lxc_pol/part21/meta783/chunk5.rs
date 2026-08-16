//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2814/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2814(t10073: f64, t14537: f64, t51653: f64, t51657: f64, t51660: f64, t51668: f64, t51672: f64, t51676: f64, t51680: f64, t51683: f64, t51685: f64, t51686: f64) -> f64 {
    let t51688 = t10073 * t14537;
    let t51690 = 0.32927245914677557992e-1_f64 * t51653 - 0.7805952431506226415e-1_f64 * t51657 + 0.19637199382202157274e-3_f64 * t51660 - 0.17563392970889009433e0_f64 * t51668 + 0.16463622957338778996e-1_f64 * t51672 - 0.19637199382202157274e-3_f64 * t51676 + 0.32927245914677557992e-1_f64 * t51680 + t51683 - t51685 + 0.17073386770573548589e-1_f64 * t51686 + 0.19514881078765566037e-2_f64 * t51688;
    t51690
}
