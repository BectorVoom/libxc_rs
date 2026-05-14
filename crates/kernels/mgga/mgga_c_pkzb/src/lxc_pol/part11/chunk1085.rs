//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1085/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1085<F: Float>(t1029: F, t10502: F, t10556: F, t10612: F, t10615: F, t10618: F, t160: F, t24064: F, t2575: F, t2631: F, t2632: F, t29067: F, t3396: F, t5304: F, t568: F, t594: F, t596: F, t614: F, t7065: F, t7070: F, t7074: F, t8817: F, t8872: F, t8885: F) -> (F,) {
    let t29209 = -360.0 * t10502 * t2631 * t5304 * t568 - 12.0 * t10556 * t2631 * t568 * t614 + 3.0 * t160 * t29067 * t596 + 180.0 * t24064 * t2631 * t7070 + 180.0 * t2575 * t2631 * t8872 - 36.0 * t2631 * t2632 * t8817 - 36.0 * t2631 * t3396 * t7074 + 9.0 * t1029 * t8885 + 60.0 * t10612 * t594 - 36.0 * t10615 * t7065 + 3.0 * t10618 * t594;
    (t29209,)
}
