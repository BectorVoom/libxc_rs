//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1074/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1074<F: Float>(t13589: F, t4882: F, t1737: F, t2471: F, t1742: F, t2475: F, t1734: F, t2466: F, t1068: F, t1646: F, t10108: F, t1056: F, t13475: F) -> (F, F, F, F, F, F, F) {
    let t13590 = t13589 * t4882;
    let t13592 = t2471 * t1737;
    let t13594 = t2475 * t1742;
    let t13596 = t2466 * t1734;
    let t13598 = t1068 * t1646;
    let t13600 = t10108 * t1646;
    let t13602 = t1056 * t13475;
    (t13590, t13592, t13594, t13596, t13598, t13600, t13602)
}
