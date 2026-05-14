//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1280/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1280<F: Float>(t39810: F, t403: F, t110289: F, t9446: F, t32026: F, t32176: F, t110284: F, t32102: F, t1308: F, t3930: F, t4153: F, t32025: F, t3969: F, t1292: F, t13437: F, t32143: F, t9442: F) -> (F, F, F, F, F, F, F, F) {
    let t110435 = t403 * t39810;
    let t110440 = t9446 * t110289;
    let t110443 = t32026 * t32176;
    let t110445 = t32102 * t110284;
    let t110452 = t3930 * t4153 * t1308;
    let t110459 = t32025 * t3969;
    let t110463 = t13437 * t1292 * t1308;
    let t110466 = t32143 * t9442;
    (t110435, t110440, t110443, t110445, t110452, t110459, t110463, t110466)
}
