//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1077/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1077<F: Float>(t6595: F, t797: F, t5037: F, t732: F, t1524: F, t1696: F, t1419: F, t1512: F, t1481: F, t1484: F, t14: F, t18938: F) -> (F, F, F, F, F) {
    let t18996 = t6595 * t797;
    let t19000 = t732 * t5037;
    let t19002 = t1696 * t1524;
    let t19004 = t1419 * t1512;
    let t19006 = t1481 * t1481;
    let t19009 = t1484 * t1484;
    let t19013 = 0.24955700379505800916e5 * t14 / t19006 * t18938 / t19009;
    (t18996, t19000, t19002, t19004, t19013)
}
