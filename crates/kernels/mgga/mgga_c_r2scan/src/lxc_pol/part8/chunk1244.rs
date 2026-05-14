//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1244/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1244<F: Float>(t537: F, t8691: F, t2294: F, t2582: F, t9151: F, t8629: F, t7512: F, t8803: F, t2133: F, t9139: F, t6152: F, t9387: F, t22709: F, t6132: F, t8745: F, t6139: F, t8741: F) -> (F, F, F, F, F, F, F, F) {
    let t27661 = t537 * t8691;
    let t27678 = t2582 * t2294 * t9151;
    let t27688 = t537 * t8629;
    let t27725 = t7512 * t2294 * t8803;
    let t27736 = t2133 * t2294 * t9139;
    let t27738 = t6152 * t9387;
    let t27741 = t6132 * t22709 * t8745;
    let t27744 = t6139 * t22709 * t8741;
    (t27661, t27678, t27688, t27725, t27736, t27738, t27741, t27744)
}
