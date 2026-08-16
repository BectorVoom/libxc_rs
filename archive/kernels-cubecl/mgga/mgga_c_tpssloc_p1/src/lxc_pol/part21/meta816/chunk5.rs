//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2878/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2878<F: Float>(t60106: F, t60120: F, t60133: F, t60147: F, t901: F, t42444: F, t48140: F, t55716: F, t43317: F, t41656: F, t41658: F, t41675: F, t41684: F, t41863: F, t41870: F, t41872: F, t47738: F, t48103: F, t48116: F, t59655: F, t60091: F) -> (F, F, F, F, F) {
    let t60149 = t60106 + t60120 + t60133 + t60147;
    let t60150 = t901 * t60149;
    let t60153 = t48140 * t42444 * t55716;
    let t60156 = t48140 * t43317 * t55716;
    let t60158 = F::cast_from(0.12077e1_f64) * t47738 + F::cast_from(0.49057777777777777779e0_f64) * t48103 - F::cast_from(0.13418888888888888889e0_f64) * t41656 - F::cast_from(0.8945925925925925926e-1_f64) * t41658 + F::cast_from(0.26837777777777777778e0_f64) * t41675 + F::cast_from(0.62621481481481481482e0_f64) * t41684 + F::cast_from(0.49057777777777777778e0_f64) * t41863 - F::cast_from(0.91983333333333333333e-1_f64) * t41870 - F::cast_from(0.30661111111111111111e-1_f64) * t41872 + F::cast_from(0.49057777777777777777e-1_f64) * t48116 - F::cast_from(0.198684e1_f64) * t60091 - F::cast_from(0.72462e1_f64) * t59655 + F::cast_from(0.16504875e0_f64) * t60150 + F::cast_from(0.66228e0_f64) * t60153 - F::cast_from(0.14717333333333333333e0_f64) * t60156;
    (t60149, t60150, t60153, t60156, t60158)
}
