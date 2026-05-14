//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1021/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1021<F: Float>(t10760: F, t25307: F, t6093: F, t19865: F, t25314: F, t261: F, t3304: F, t7239: F, t1054: F, t6583: F, t7326: F, t10799: F, t2207: F, t3613: F, t10814: F, t2651: F) -> (F, F, F, F, F, F) {
    let t39655 = t6093 * t10760 * t25307;
    let t39658 = t19865 * t10760 * t25314;
    let t39661 = t3304 * t261 * t7239;
    let t39664 = t6583 * t1054 * t7326;
    let t39667 = t2207 * t3613 * t10799;
    let t39669 = t2651 * t10814;
    (t39655, t39658, t39661, t39664, t39667, t39669)
}
