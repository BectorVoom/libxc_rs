//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1201/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1201<F: Float>(t32203: F, t9814: F, t1339: F, t220: F, t3797: F, t9461: F, t5600: F, t388: F, t6221: F, t1308: F) -> (F, F, F, F, F, F, F) {
    let t33363 = t32203 * t9814;
    let t33364 = t1339 * t33363;
    let t33366 = t3797 * t220;
    let t33367 = t9461 * t33366;
    let t33368 = t5600 * t33367;
    let t33372 = t6221 * t388;
    let t33373 = t33372 * t1308;
    (t33363, t33364, t33366, t33367, t33368, t33372, t33373)
}
