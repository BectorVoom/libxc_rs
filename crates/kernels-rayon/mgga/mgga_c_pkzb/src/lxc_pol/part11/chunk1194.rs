//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1194/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1194(t1029: f64, t10502: f64, t10556: f64, t10612: f64, t10615: f64, t10618: f64, t160: f64, t24064: f64, t2575: f64, t2631: f64, t2632: f64, t29067: f64, t3396: f64, t5304: f64, t568: f64, t594: f64, t596: f64, t614: f64, t7065: f64, t7070: f64, t7074: f64, t8817: f64, t8872: f64, t8885: f64) -> f64 {
    let t29209 = -360.0_f64 * t10502 * t2631 * t5304 * t568 - 12.0_f64 * t10556 * t2631 * t568 * t614 + 3.0_f64 * t160 * t29067 * t596 + 180.0_f64 * t24064 * t2631 * t7070 + 180.0_f64 * t2575 * t2631 * t8872 - 36.0_f64 * t2631 * t2632 * t8817 - 36.0_f64 * t2631 * t3396 * t7074 + 9.0_f64 * t1029 * t8885 + 60.0_f64 * t10612 * t594 - 36.0_f64 * t10615 * t7065 + 3.0_f64 * t10618 * t594;
    t29209
}
