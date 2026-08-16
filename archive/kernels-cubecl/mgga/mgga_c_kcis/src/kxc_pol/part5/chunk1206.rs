//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1206/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1206<F: Float>(t1094: F, t6680: F, t1172: F, t19619: F, t5047: F, t5046: F, t14785: F, t5073: F, t19856: F, t3338: F, t10526: F, t6690: F, sigma0: F) -> (F, F, F, F, F) {
    let t20155 = t6680 * t1094;
    let t20156 = t20155 * sigma0;
    let t20157 = t20156 * t1172;
    let t20159 = t5047 * t19619;
    let t20160 = t5046 * t20159;
    let t20162 = t14785 * t5073;
    let t20164 = t3338 * t19856;
    let t20165 = t5046 * t20164;
    let t20167 = t10526 * t6690;
    (t20157, t20160, t20162, t20165, t20167)
}
