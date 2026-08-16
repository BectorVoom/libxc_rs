//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1802/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1802<F: Float>(t19871: F, t5248: F, t5250: F, t5234: F, t5245: F, t12283: F, t6396: F, t3805: F, t3807: F, t16306: F, t6394: F, t16305: F) -> (F, F, F, F, F) {
    let t19873 = t5248 * t19871 * t5250;
    let t19876 = t5234 * t5245;
    let t19879 = t12283 * t6396;
    let t19882 = t3805 * t19871 * t3807;
    let t19885 = t16306 * t6394;
    let t19886 = t16305 * t19885;
    (t19873, t19876, t19879, t19882, t19886)
}
