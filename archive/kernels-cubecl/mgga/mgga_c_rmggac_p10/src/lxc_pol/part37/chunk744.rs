//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 744/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk744<F: Float>(t70082: F, t14563: F, t2160: F, t638: F, t14559: F, t70237: F, t14580: F, t899: F, t70328: F, t70376: F, t70385: F, t70439: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71672 = F::cast_from(0.30487649791575028312e-3_f64) * t70082;
    let t71717 = t638 * t2160 * t14563;
    let t71720 = t638 * t2160 * t14559;
    let t71744 = F::cast_from(0.60975299583150056624e-3_f64) * t70237;
    let t71772 = t899 * t14580;
    let t71789 = F::cast_from(0.3830813990396805546e-3_f64) * t70328;
    let t71802 = F::cast_from(0.162600798888400151e-2_f64) * t70376;
    let t71804 = F::cast_from(0.32526727992809621482e-4_f64) * t70385;
    let t71832 = F::cast_from(0.2316441583394736328e-4_f64) * t70439;
    (t71672, t71717, t71720, t71744, t71772, t71789, t71802, t71804, t71832)
}
