//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1224/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1224<F: Float>(t15793: F, t236: F, t233: F, t1885: F, t4504: F, t446: F, t12274: F, t2003: F, t1396: F, t531: F, t1395: F, t5780: F) -> (F, F, F, F) {
    let t15794 = t236 * t15793;
    let t15795 = t233 * t15794;
    let t15797 = t1885 * t4504;
    let t15798 = t446 * t15797;
    let t15800 = t12274 * t2003;
    let t15802 = t1396 * t531;
    let t15803 = t1395 * t15802;
    let t15804 = t5780 * t15803;
    (t15795, t15798, t15800, t15804)
}
