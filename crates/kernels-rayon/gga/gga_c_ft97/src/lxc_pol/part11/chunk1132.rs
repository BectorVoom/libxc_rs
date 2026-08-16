//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1132/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1132(t2735: f64, t820: f64, t2719: f64, t10292: f64, t10296: f64, t10297: f64, t10358: f64, t10359: f64, t10363: f64, t10365: f64, t10369: f64, t10384: f64, t14763: f64, t2688: f64, t2691: f64, t2692: f64, t2724: f64, t2725: f64, t2726: f64, t2727: f64, t2736: f64, t284: f64, t285: f64, t291: f64, t4061: f64, t4113: f64, t43586: f64, t43587: f64, t43595: f64, t43626: f64, t43639: f64, t43702: f64, t43766: f64, t800: f64, t811: f64, t816: f64, t817: f64) -> f64 {
    let t43777 = t820 * t2735;
    let t43781 = t2719 * t2719;
    let t43789 = 24.0_f64 * t285 * t43586 * t43587 - 36.0_f64 * t4113 * t10363 * t2726 * t2735 + 6.0_f64 * t285 * t2725 * t43595 + 8.0_f64 * t4113 * t10369 * t10384 - 12.0_f64 * t2691 * t10296 * t2735 + 24.0_f64 * t2688 * t2727 + 24.0_f64 * t2691 * t2724 * t2719 * t2726 - 8.0_f64 * t2691 * t816 * t10358 * t820 - t285 * t817 * (t43626 + t43639) - 8.0_f64 * t2691 * t2692 * t10384 + 2.0_f64 * t800 * t291 * (t43702 + t43766) - 48.0_f64 * t2691 * t10363 * t811 * t10365 - 24.0_f64 * t14763 * t10297 + 48.0_f64 * t2691 * t10292 * t43777 + 6.0_f64 * t43781 * t284 * t291 + 8.0_f64 * t4061 * t10359 - 12.0_f64 * t2688 * t2736;
    t43789
}
