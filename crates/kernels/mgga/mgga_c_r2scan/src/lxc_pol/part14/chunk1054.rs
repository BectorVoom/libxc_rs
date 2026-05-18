//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1054/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1054<F: Float>(t166: F, t874: F, t37426: F, t37427: F, t424: F, t10645: F, t10976: F, t2104: F, t3437: F, t58: F, t10929: F, t3428: F, t3430: F, t6826: F, t761: F) -> (F, F, F, F, F, F) {
    let t37428 = t166 * t874;
    let t37431 = t37426 * t37427 * t424 * t37428;
    let t37434 = t10645 * t10976 * t2104;
    let t37435 = t3437 * t58;
    let t37438 = t37434 * t37435 * t424 * t10929;
    let t37442 = t6826 * t761 * t3428 * t3430;
    (t37428, t37431, t37434, t37435, t37438, t37442)
}
