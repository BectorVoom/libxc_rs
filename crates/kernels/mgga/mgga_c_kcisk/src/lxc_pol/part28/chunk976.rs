//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 976/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk976<F: Float>(t827: F, t8567: F, t8570: F, t1653: F, t22596: F, t10621: F, t22392: F, t26: F, t22396: F, t4726: F, t5744: F, t1659: F, t22484: F, t22488: F, t22501: F, t22506: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22705 = t827 * t8567;
    let t22707 = t827 * t8570;
    let t22711 = t1653 * t22596;
    let t22713 = t10621 * t22392;
    let t22714 = t26 * t22713;
    let t22717 = t4726 * t22396;
    let t22718 = t5744 * t22717;
    let t22720 = t1659 * t22484;
    let t22721 = t26 * t22720;
    let t22723 = t1659 * t22488;
    let t22724 = t5744 * t22723;
    let t22726 = t1659 * t22501;
    let t22727 = t26 * t22726;
    let t22729 = t4726 * t22506;
    (t22705, t22707, t22711, t22714, t22718, t22721, t22724, t22727, t22729)
}
