//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1152/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1152(t232: f64, t2646: f64, t4180: f64, t30714: f64, t235: f64, t835: f64, t226: f64, t8344: f64, t8343: f64, t849: f64, t30698: f64, t30701: f64, t30705: f64, t30707: f64, t30710: f64) -> (f64, f64, f64, f64, f64) {
    let t30716 = t4180 * t2646 * t232;
    let t30717 = t30714 * t30716;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30722 = 7.0_f64 / 2304.0_f64 * t30721;
    let t30723 = t8343 * t849;
    let t30725 = -t30698 - 0.48447307312968469025e-2_f64 * t30701 - t30705 - 0.80745512188280781708e-3_f64 * t30707 + t30710 / 1536.0_f64 - t30717 / 1536.0_f64 - t30722 - t30723 / 384.0_f64;
    (t30716, t30719, t30720, t30722, t30725)
}
