//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1430/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1430<F: Float>(t2049: F, t2461: F, t759: F, t19069: F, t19341: F, t19388: F, t19394: F, t23781: F, t23796: F, t23798: F, t23801: F, t26938: F, t26941: F, t2056: F, t2768: F, t6028: F) -> (F, F) {
    let t26944 = t759 * t2461 * t2049;
    let t26945 = 0.857292e-1 * t26944;
    let t26946 = -t19069 - t23781 + t19341 + 0.4051561992e0 * t26938 + t23796 + t23798 + 0.857292e-1 * t26941 + t26945 + t19388 + t19394 + t23801;
    let t26947 = t2768 * t2056;
    let t26948 = t6028 * t26947;
    (t26946, t26948)
}
