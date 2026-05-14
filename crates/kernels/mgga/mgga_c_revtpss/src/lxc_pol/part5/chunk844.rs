//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 844/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk844<F: Float>(t225: F, t385: F, t6343: F, t1695: F, t3269: F, t1082: F, t6244: F, t1089: F, t6271: F, t1651: F, t5004: F, t6258: F, t378: F, t6305: F, t3304: F, t1668: F, t1678: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6345 = t6343 * t225 * t385;
    let t6350 = t1695 * t1695;
    let t6351 = t3269 * t6350;
    let t6362 = t1082 * t6244;
    let t6365 = t6271 * t1089;
    let t6368 = t5004 * t1651;
    let t6371 = t1082 * t6258;
    let t6374 = t378 * t6305;
    let t6375 = t6374 * t3304;
    let t6379 = t1678 * t1668 * t1089;
    (t6345, t6350, t6351, t6362, t6365, t6368, t6371, t6374, t6375, t6379)
}
