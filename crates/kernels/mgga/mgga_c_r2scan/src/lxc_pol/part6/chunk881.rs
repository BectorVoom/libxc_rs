//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 881/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk881<F: Float>(t551: F, t552: F, t6219: F, t5066: F, t1572: F, t1600: F, t1570: F, t1632: F, t1577: F, t1593: F, t1592: F, t2090: F, t57: F) -> (F, F, F, F, F, F, F, F) {
    let t6221 = t551 * t552 * t6219;
    let t6225 = t551 * t552 * t5066;
    let t6228 = t1600 * t1572;
    let t6231 = t551 * t1632 * t1570;
    let t6232 = t1577 * t6231;
    let t6235 = t551 * t1632 * t1593;
    let t6236 = t1592 * t6235;
    let t6238 = t2090 * t57;
    (t6221, t6225, t6228, t6231, t6232, t6235, t6236, t6238)
}
