//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 382/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk382<F: Float>(t1214: F, t482: F, t372: F, t371: F, t1032: F, t460: F, t472: F, t474: F, t1038: F, t479: F) -> (F, F, F, F, F, F, F) {
    let t1236 = t482 * t1214;
    let t1237 = t372 * t1236;
    let t1238 = t371 * t1237;
    let t1241 = t460 * t1032;
    let t1242 = t472 * t472;
    let t1243 = F::cast_from(1.0_f64) / t1242;
    let t1244 = t1243 * t474;
    let t1245 = t479 * t1038;
    let t1246 = t1244 * t1245;
    (t1236, t1238, t1241, t1242, t1243, t1244, t1246)
}
