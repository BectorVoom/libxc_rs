//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 497/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk497<F: Float>(t123: F, t265: F, t200: F, t220: F, t721: F, t754: F, t772: F, t132: F, t776: F, t780: F, t744: F, t2607: F, t2610: F, t2655: F, t2658: F, t2660: F, t2663: F, t2669: F, t2695: F, t2696: F, t273: F, t4: F, t71: F, t748: F, t786: F, t792: F, t796: F, t799: F, t800: F, t84: F) -> (F, F, F, F) {
    let t2700 = t123 * t265;
    let t2707 = t123 * t200;
    let t2709 = t721 * t2707 * t220;
    let t2710 = F::cast_from(0.71233333333333333332e-1_f64) * t2709;
    let t2712 = t721 * t754 * t772;
    let t2713 = F::cast_from(0.53424999999999999999e-1_f64) * t2712;
    let t2714 = t132 * t776;
    let t2716 = t721 * t2714 * t780;
    let t2717 = F::cast_from(0.85917975471764868594e0_f64) * t2716;
    let t2718 = t132 * t744;
    let t2722 = F::cast_from(0.56968947174242584612e-3_f64) * t4 * t2607 * t84 - t2610 + F::cast_from(0.16562821945185185185e-2_f64) * t4 * t2607 * t71 - t2655 + t2658 + F::cast_from(0.51947577317044391277e2_f64) * t799 * t2660 - F::cast_from(0.35089341735807877242e1_f64) * t792 * t2663 - t2669 - t2695 - F::cast_from(0.48159733137676571078e0_f64) * t721 * t2696 * t800 + F::cast_from(0.21687162600603479684e-1_f64) * t721 * t2700 * t273 - F::cast_from(0.16265371950452609763e-1_f64) * t721 * t786 * t796 - t2710 + t2713 + t2717 - F::cast_from(0.16522625736956710527e1_f64) * t721 * t2718 * t748;
    (t2710, t2713, t2717, t2722)
}
