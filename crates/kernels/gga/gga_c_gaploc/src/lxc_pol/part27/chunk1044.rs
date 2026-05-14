//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1044/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1044<F: Float>(t1651: F, t3366: F, t4349: F, t27214: F, t6565: F, t6568: F, t8045: F, t2798: F, t7058: F, t6556: F, t8060: F, t2497: F, t8042: F, t8057: F, t10301: F, t4342: F) -> (F, F, F, F, F, F, F, F) {
    let t31461 = 6.0 * t4349 * t3366 * t1651;
    let t31463 = 6.0 * t27214 * t6565;
    let t31465 = 4.0 * t8045 * t6568;
    let t31470 = t2798 * t7058;
    let t31472 = 2.0 * t6556 * t8060;
    let t31474 = 2.0 * t8042 * t2497;
    let t31476 = 4.0 * t6556 * t8057;
    let t31480 = 4.0 * t4342 * t10301;
    (t31461, t31463, t31465, t31470, t31472, t31474, t31476, t31480)
}
