//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 794/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk794<F: Float>(t159: F, t7778: F, t617: F, t1678: F, t955: F, t1686: F, t2035: F, t898: F, t41: F, t5883: F, t5885: F, t1745: F, t963: F) -> (F, F, F, F, F, F) {
    let t7779 = t159 * t7778;
    let t7781 = F::cast_from(0.16936279733333333333e-2_f64) * t7779 * t617;
    let t7783 = t955 * t1678;
    let t7784 = t159 * t7783;
    let t7785 = t7784 * t1686;
    let t7794 = t898 * t2035;
    let t7795 = t41 * t7794;
    let t7796 = F::cast_from(4.0_f64) * t5883;
    let t7797 = F::cast_from(12.0_f64) * t5885;
    let t7798 = t963 * t1745;
    (t7781, t7785, t7795, t7796, t7797, t7798)
}
