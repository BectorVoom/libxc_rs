//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1374/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1374<F: Float>(t23669: F, t995: F, t6802: F, t3158: F, t6796: F, t10481: F, t1945: F, t23665: F, t23674: F, t23600: F, t23680: F, t23606: F) -> (F, F, F, F, F, F) {
    let t82713 = t23669 * t995;
    let t82714 = t82713 * t6802;
    let t82716 = t6796 * t3158;
    let t82717 = t82716 * t6802;
    let t82730 = t1945 * t10481;
    let t82734 = t23665 * t23674;
    let t82736 = t23600 * t995;
    let t82737 = t82736 * t23680;
    let t82739 = t82736 * t23606;
    (t82714, t82717, t82730, t82734, t82737, t82739)
}
