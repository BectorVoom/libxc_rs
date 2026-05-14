//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1020/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1020<F: Float>(t18036: F, t3290: F, t5248: F, t140: F, t1470: F, t16872: F, t17993: F, t17996: F, t18001: F, t18005: F, t18022: F, t18026: F, t18031: F, t18033: F, t1909: F, t2517: F, t2521: F, t2543: F, t3077: F, t4653: F, t4685: F, t479: F, t5222: F, t7039: F, t7060: F, t725: F, t7349: F) -> (F,) {
    let t18038 = t5248 * t18036 * t3290;
    let t18041 = 0.53062222222222222222e-1 * t1470 * t17993 - 0.39796666666666666666e-1 * t140 * t479 * t17996 - 0.9286875e-2 * t7349 * t18001 + 0.88437037037037037037e-2 * t18005 + 0.9286875e-2 * t2543 * t4653 - 0.619125e-2 * t2543 * t4685 + 0.9286875e-2 * t5222 * t2517 + 0.1857375e-1 * t1909 * t7039 - 0.619125e-2 * t5222 * t2521 - 0.123825e-1 * t1909 * t7060 - 0.619125e-2 * t725 * t16872 - 0.26531111111111111111e-1 * t1470 * t18022 + 0.10612444444444444444e0 * t3077 * t18026 - t18031 - 0.26531111111111111111e-1 * t1470 * t18033 - 0.44218518518518518518e-1 * t1470 * t18038;
    (t18041,)
}
