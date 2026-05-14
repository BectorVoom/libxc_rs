//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1044/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1044<F: Float>(t7580: F, t92247: F, t7589: F, t26580: F, t26623: F, t2140: F, t2381: F, t3110: F, t1075: F, t9232: F, t26597: F, t26615: F, t7579: F, t7592: F, t7583: F, t36962: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92256 = t7580 * t92247;
    let t92258 = t7589 * t92247;
    let t92260 = t26580 * t26623;
    let t92263 = t2381 * t3110 * t2140;
    let t92266 = t9232 * t1075 * t2140;
    let t92268 = t26597 * t26615;
    let t92270 = t9232 * t7579;
    let t92271 = t92270 * t7592;
    let t92273 = t92270 * t7583;
    let t92276 = t36962 * t7579 * t7583;
    (t92256, t92258, t92260, t92263, t92266, t92268, t92271, t92273, t92276)
}
