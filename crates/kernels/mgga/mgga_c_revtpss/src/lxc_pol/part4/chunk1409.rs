//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1409/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1409<F: Float>(t17879: F, t460: F, t3584: F, t5457: F, t5351: F, t1269: F, t3766: F, t1280: F, t17345: F, t1287: F, t17389: F, t17600: F) -> (F, F, F, F, F, F) {
    let t17880 = t460 * t17879;
    let t17883 = t5457 * t3584;
    let t17884 = t5351 * t17883;
    let t17887 = t3766 * t1269;
    let t17888 = t460 * t17887;
    let t17893 = t1280 * t17345;
    let t17902 = t17389 * t1287;
    let t17905 = t17600 * t1287;
    (t17880, t17884, t17888, t17893, t17902, t17905)
}
