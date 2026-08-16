//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 782/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk782<F: Float>(t1100: F, t1954: F, t7335: F, t2848: F, t237: F, t2826: F, t1125: F, t5939: F, t757: F, t2096: F, t2908: F, t2886: F, t434: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7494 = t1100 * t1954;
    let t7500 = F::cast_from(0.35616666666666666666e-1_f64) * t7335;
    let t7508 = F::cast_from(0.18541666666666666667e-1_f64) * t7335;
    let t7516 = F::cast_from(0.34246666666666666666e-1_f64) * t7335;
    let t7527 = t1954 * t2848;
    let t7560 = t237 * t2826;
    let t7581 = t5939 * t1125;
    let t7582 = t757 * t7581;
    let t7585 = F::cast_from(0.15244095330869239812e-2_f64) * t2096 * t2908;
    let t7586 = t434 * t2886;
    (t7494, t7500, t7508, t7516, t7527, t7560, t7581, t7582, t7585, t7586)
}
