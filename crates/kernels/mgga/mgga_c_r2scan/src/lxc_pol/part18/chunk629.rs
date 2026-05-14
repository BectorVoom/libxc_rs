//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 629/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk629<F: Float>(t1053: F, t1102: F, t3692: F, t3432: F, t3442: F, t3445: F, t3451: F, t3577: F, t3581: F, t3585: F, t3621: F, t3624: F, t3690: F, t273: F, t57: F, t1064: F, t2333: F) -> (F, F, F) {
    let t3694 = t1102 * t1053 * t3692;
    let t3696 = -t3432 + t3442 - t3445 - t3451 - 0.36021158228745895953e-3 * t3690 + 0.15243824895787514157e-3 * t3694 - t3577 - t3581 + t3585 - t3621 + t3624;
    let t4145 = t57 * t273;
    let t4176 = t2333 * t1064;
    (t3696, t4145, t4176)
}
