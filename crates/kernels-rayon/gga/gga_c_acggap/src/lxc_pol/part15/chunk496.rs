//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 496/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk496(t123: f64, t265: f64, t200: f64, t220: f64, t721: f64, t754: f64, t772: f64, t132: f64, t776: f64, t780: f64, t744: f64, t2607: f64, t2610: f64, t2655: f64, t2658: f64, t2660: f64, t2663: f64, t2669: f64, t2695: f64, t2696: f64, t273: f64, t4: f64, t71: f64, t748: f64, t786: f64, t792: f64, t796: f64, t799: f64, t800: f64, t84: f64) -> (f64, f64, f64, f64) {
    let t2700 = t123 * t265;
    let t2707 = t123 * t200;
    let t2709 = t721 * t2707 * t220;
    let t2710 = 0.71233333333333333332e-1_f64 * t2709;
    let t2712 = t721 * t754 * t772;
    let t2713 = 0.53424999999999999999e-1_f64 * t2712;
    let t2714 = t132 * t776;
    let t2716 = t721 * t2714 * t780;
    let t2717 = 0.85917975471764868594e0_f64 * t2716;
    let t2718 = t132 * t744;
    let t2722 = 0.56968947174242584612e-3_f64 * t4 * t2607 * t84 - t2610 + 0.16562821945185185185e-2_f64 * t4 * t2607 * t71 - t2655 + t2658 + 0.51947577317044391277e2_f64 * t799 * t2660 - 0.35089341735807877242e1_f64 * t792 * t2663 - t2669 - t2695 - 0.48159733137676571078e0_f64 * t721 * t2696 * t800 + 0.21687162600603479684e-1_f64 * t721 * t2700 * t273 - 0.16265371950452609763e-1_f64 * t721 * t786 * t796 - t2710 + t2713 + t2717 - 0.16522625736956710527e1_f64 * t721 * t2718 * t748;
    (t2710, t2713, t2717, t2722)
}
