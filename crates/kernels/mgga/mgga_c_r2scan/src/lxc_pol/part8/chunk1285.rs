//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1285/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1285<F: Float>(t1577: F, t3071: F, t551: F, t6343: F, t1592: F, t1632: F, t9103: F, t2562: F, t2719: F, t2148: F, t7628: F, t22709: F, t5108: F, t8769: F, t5147: F, t5148: F, t9376: F) -> (F, F, F, F, F) {
    let t30189 = t1577 * t551 * t6343 * t3071;
    let t30205 = t1592 * t551 * t1632 * t9103;
    let t30213 = t2562 * t2719;
    let t30215 = t7628 * t2148 * t30213;
    let t30218 = t5108 * t22709 * t8769;
    let t30233 = t5147 * t5148 * t9376;
    (t30189, t30205, t30215, t30218, t30233)
}
