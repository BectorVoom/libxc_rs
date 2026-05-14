//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1231/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1231<F: Float>(t1111: F, t1133: F, t27096: F, t27101: F, t27105: F, t27110: F, t27113: F, t27119: F, t27124: F, t27127: F, t27131: F, t27135: F, t27138: F, t27141: F, t322: F, t2856: F, t4356: F) -> (F, F) {
    let t27143 = -t1111 * t322 * t27096 / 48.0 + t1111 * t322 * t27101 / 6.0 + t1111 * t322 * t27105 / 72.0 + t27110 / 36.0 - t1111 * t322 * t27113 / 12.0 + 0.73258227843678641352e2 * t27119 + 0.18933502127510156893e0 * t27124 + 0.48295341609937543636e-2 * t27127 - 0.96590683219875087274e-1 * t1133 * t27131 - 0.40246118008281286364e-2 * t27135 - 0.48295341609937543636e-1 * t27138 - 0.47333755318775392234e-1 * t27141;
    let t27148 = t4356 * t2856;
    (t27143, t27148)
}
