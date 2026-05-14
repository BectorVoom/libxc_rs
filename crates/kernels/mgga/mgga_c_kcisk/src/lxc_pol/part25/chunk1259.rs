//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1259/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1259<F: Float>(t32909: F, t32942: F, t32990: F, t32995: F, t32904: F, t5074: F, t33041: F, t10494: F, t33045: F, t5306: F, t654: F, t62249: F, t9651: F, t9664: F, t17182: F, t33004: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t112460 = t32942 * t32909;
    let t112462 = t32990 * t32909;
    let t112502 = t32990 * t32995;
    let t112506 = t5074 * t32904;
    let t112508 = t5074 * t33041;
    let t112510 = t10494 * t33045;
    let t112512 = t5306 * t654;
    let t112517 = t62249 * t9651;
    let t112518 = t9664 * t112517;
    let t112520 = t17182 * t33004;
    (t112460, t112462, t112502, t112506, t112508, t112510, t112512, t112517, t112518, t112520)
}
