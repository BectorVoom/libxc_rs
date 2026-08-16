//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1213/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1213(t37749: f64, t37759: f64, t37762: f64, t39655: f64, t39658: f64, t39661: f64, t39664: f64, t39667: f64, t39669: f64, t39672: f64, t39674: f64, t39677: f64) -> f64 {
    let t41498 = -0.13099107994629972538e-1_f64 * t39655 + 0.87327386630866483588e-2_f64 * t39658 - 0.2600466522016280569e0_f64 * t39661 - 0.34672886960217074252e0_f64 * t39664 + 0.13099107994629972538e-1_f64 * t39667 - 0.86682217400542685632e-1_f64 * t39669 - 0.13869154784086829701e1_f64 * t37749 - 0.51220160311720645767e0_f64 * t39672 + 0.5200933044032561138e0_f64 * t39674 - 0.5200933044032561138e0_f64 * t39677 - 0.23804984598836975486e0_f64 * t37759 + 0.47609969197673950973e-2_f64 * t37762;
    t41498
}
