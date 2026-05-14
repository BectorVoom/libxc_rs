//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1245/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1245<F: Float>(t5257: F, t9005: F, t16402: F, t3413: F, t3444: F, t5384: F, t1029: F, t1031: F, t160: F, t1634: F, t1692: F, t1742: F, t1746: F, t1747: F, t24091: F, t2631: F, t2632: F, t3401: F, t3431: F, t3435: F, t3438: F, t5304: F, t594: F, t597: F, t6853: F, t7055: F, t7065: F, t7071: F, t7081: F, t8859: F, t8865: F, t8872: F, t8873: F, t8885: F) -> (F, F, F, F) {
    let t24461 = t5257 * t9005;
    let t24487 = t16402 * t3413;
    let t24489 = t5384 * t3444;
    let t24523 = -360.0 * t1634 * t2631 * t3401 * t5304 - 24.0 * t160 * t1746 * t24091 + 60.0 * t1692 * t2631 * t8872 - 24.0 * t2631 * t2632 * t6853 + 6.0 * t1029 * t7081 + 6.0 * t1031 * t7055 - 12.0 * t1742 * t3435 + 3.0 * t1742 * t3438 - 12.0 * t1747 * t3431 + 6.0 * t594 * t8885 + 6.0 * t597 * t8859 + 120.0 * t7065 * t8873 + 120.0 * t7071 * t8865;
    (t24461, t24487, t24489, t24523)
}
