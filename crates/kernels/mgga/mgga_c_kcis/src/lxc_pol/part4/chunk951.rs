//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 951/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk951<F: Float>(t4797: F, t9429: F, t1769: F, t9528: F, t2861: F, t5020: F, t5010: F, t2825: F, t5013: F, t1092: F, t5014: F, t10250: F, t1773: F, t1021: F, t3220: F, t4999: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13301 = t9429 * t4797;
    let t13302 = 0.14739506172839506172e-2 * t13301;
    let t13303 = t9528 * t1769;
    let t13305 = t2861 * t5020;
    let t13307 = t2861 * t5010;
    let t13308 = 0.22109259259259259258e-2 * t13307;
    let t13309 = t2825 * t5013;
    let t13310 = t1092 * t13309;
    let t13312 = t2861 * t5014;
    let t13314 = t10250 * t1773;
    let t13315 = t1021 * t13314;
    let t13316 = t1092 * t13315;
    let t13318 = t4999 * t3220;
    (t13301, t13302, t13303, t13305, t13307, t13308, t13310, t13312, t13316, t13318)
}
