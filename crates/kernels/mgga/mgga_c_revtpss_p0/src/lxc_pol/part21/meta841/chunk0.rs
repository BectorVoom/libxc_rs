//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3153/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3153<F: Float>(t56228: F, t1132: F, t58106: F, t1134: F, t3399: F, t16851: F, t16854: F, t2439: F, t5101: F, t16870: F, t698: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F) -> (F, F, F, F, F, F) {
    let t58134 = F::cast_from(0.68863333333333333332e0_f64) * t56228;
    let t58138 = t1132 * t58106;
    let t58140 = t1134 * t3399;
    let t58141 = t16851 * t58140;
    let t58143 = t16854 * t58140;
    let t58145 = t2439 * t5101;
    let t58146 = F::cast_from(0.34731666666666666667e0_f64) * t58145;
    let t58147 = t698 * t16870;
    let t58149 = F::cast_from(0.17215833333333333333e1_f64) * t56221 + F::new(0.309885e1) * t56226 + t58134 - F::cast_from(0.51647499999999999999e0_f64) * t56230 + F::new(0.516475e0) * t56234 - F::cast_from(0.53560370370370370369e0_f64) * t56236 + F::new(0.3529725e1) * t58138 + F::cast_from(0.794188125e1_f64) * t58141 - F::cast_from(0.473371875e0_f64) * t58143 + t58146 - F::new(0.20839e0) * t58147;
    (t58138, t58141, t58143, t58145, t58147, t58149)
}
