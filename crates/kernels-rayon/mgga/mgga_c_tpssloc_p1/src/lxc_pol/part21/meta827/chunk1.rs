//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2919/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2919(t17951: f64, t2940: f64, t14260: f64, t4483: f64, t2925: f64, t5811: f64, t959: f64, t14480: f64, t10723: f64, t17947: f64, t59637: f64, t60810: f64, t60812: f64, t60814: f64, t60816: f64, t60821: f64, t60825: f64, t60827: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60829 = 0.46785788981077169656e1_f64 * t2940 * t17951;
    let t60831 = 0.20508037716432813315e4_f64 * t4483 * t14260;
    let t60834 = 0.35089341735807877242e1_f64 * t959 * t5811 * t2925;
    let t60836 = 0.70178683471615754484e1_f64 * t4483 * t14480;
    let t60839 = 0.10389515463408878255e3_f64 * t959 * t17947 * t10723;
    let t60840 = t60810 - t60812 - t60814 + t60816 - t60821 - t60825 - t59637 - t60827 + t60829 - t60831 - t60834 - t60836 + t60839;
    (t60829, t60831, t60834, t60836, t60839, t60840)
}
