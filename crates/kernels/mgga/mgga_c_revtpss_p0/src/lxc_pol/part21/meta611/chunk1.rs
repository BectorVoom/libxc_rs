//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2363/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2363<F: Float>(t2491: F, t2495: F, t39871: F, t760: F, t10326: F, t706: F, t750: F, t2523: F, t9419: F, t10558: F, t72: F, t757: F) -> (F, F, F, F, F) {
    let t40113 = t2491 * t39871 * t2495;
    let t40115 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t40113;
    let t40119 = t706 * t750 * t10326;
    let t40121 = t2523 * t9419;
    let t40125 = t10558 * t72 * t757;
    (t40113, t40115, t40119, t40121, t40125)
}
