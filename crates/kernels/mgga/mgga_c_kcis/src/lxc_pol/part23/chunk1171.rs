//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1171/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1171<F: Float>(t7580: F, t92247: F, t7589: F, t26580: F, t26623: F, t2140: F, t2381: F, t3110: F, t1075: F, t9232: F, t26597: F, t26615: F) -> (F, F, F, F, F, F) {
    let t92256 = t7580 * t92247;
    let t92258 = t7589 * t92247;
    let t92260 = t26580 * t26623;
    let t92263 = t2381 * t3110 * t2140;
    let t92266 = t9232 * t1075 * t2140;
    let t92268 = t26597 * t26615;
    (t92256, t92258, t92260, t92263, t92266, t92268)
}
