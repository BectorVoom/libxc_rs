//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 943/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk943<F: Float>(t1211: F, t5208: F, t1823: F, t3574: F, t13908: F, t13712: F, t13714: F, t4731: F, t962: F, t1684: F, t3031: F, t3549: F, t110: F, t1852: F, t1251: F, t3490: F, t5321: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15362 = t5208 * t1211;
    let t15369 = t1823 * t3574;
    let t15397 = 0.27785333333333333334e0 * t13908;
    let t15411 = 0.22954444444444444444e0 * t13712;
    let t15432 = 0.2283111111111111111e-1 * t13714;
    let t15445 = t4731 * t962;
    let t15450 = t1684 * t3031;
    let t15460 = t1823 * t3549;
    let t15476 = t110 * t1852;
    let t15477 = t1251 * t15476;
    let t15493 = t3490 * t5321 / 108.0;
    (t15362, t15369, t15397, t15411, t15432, t15445, t15450, t15460, t15477, t15493)
}
