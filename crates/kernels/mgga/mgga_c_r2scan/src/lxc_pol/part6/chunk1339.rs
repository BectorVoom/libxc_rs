//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1339/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1339<F: Float>(t6118: F, t7341: F, t24118: F, t6121: F, t495: F, t7569: F, t2551: F, t7204: F, t6465: F, t7551: F, t1632: F, t2184: F, t551: F, t8117: F, t538: F, t910: F) -> (F, F, F, F, F, F, F) {
    let t25261 = t6118 * t7341;
    let t25263 = t24118 * t6121;
    let t25275 = t7569 * t495;
    let t25283 = t7204 * t2551;
    let t25288 = t6465 * t7551;
    let t25297 = t2184 * t551 * t1632 * t8117;
    let t25299 = t538 * t910;
    (t25261, t25263, t25275, t25283, t25288, t25297, t25299)
}
