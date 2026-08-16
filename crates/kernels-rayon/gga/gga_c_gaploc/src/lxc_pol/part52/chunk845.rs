//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 845/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk845(t45028: f64, t13542: f64, t2549: f64, t11613: f64, t1897: f64, t7675: f64, t2508: f64, t33760: f64, t9014: f64, t35719: f64, t954: f64, t44707: f64, t688: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45029 = 0.32043859292259267849e-3_f64 * t45028;
    let t45030 = t2549 * t13542;
    let t45031 = 0.32043859292259267849e-3_f64 * t45030;
    let t45034 = 0.92286314761706691403e-1_f64 * t1897 * t11613 * t7675;
    let t45037 = 0.18457262952341338281e0_f64 * t2508 * t9014 * t33760;
    let t45044 = 0.15381052460284448567e-1_f64 * t2508 * t954 * t35719;
    let t45048 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t44707 * t688;
    (t45029, t45031, t45034, t45037, t45044, t45048)
}
