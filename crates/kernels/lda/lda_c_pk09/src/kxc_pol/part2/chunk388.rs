//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 388/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk388<F: Float>(t2069: F, t467: F, t452: F, t1738: F, t1748: F, t2026: F, t2027: F, t2029: F, t2030: F, t2032: F, t2037: F, t2044: F, t2047: F, t2053: F, t2058: F, t2060: F, t453: F, t455: F, t463: F, t472: F) -> (F, F, F) {
    let t2070 = t467 * t2069;
    let t2071 = t2070 * t452;
    let t2074 = -t2026 - t2027 - 0.10237773105191754 * t1738 - t2029 - t2030 + t463 * t2032 / 6.0 - t2037 * t1748 / 6.0 + t2044 - t2047 - t472 * t2032 / 6.0 + t453 * t2032 / 6.0 - t2053 * t1748 / 6.0 + t2058 + t2060 + t2071 * t455 / 6.0;
    (t2070, t2071, t2074)
}
