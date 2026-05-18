//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 758/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk758<F: Float>(t4565: F, t4567: F, t1662: F, t3269: F, t934: F, t1045: F, t3274: F, t1103: F, t347: F, t1071: F, t1646: F) -> (F, F, F, F, F) {
    let t4568 = t4565 * t4567;
    let t4572 = t3269 * t1662 * t934;
    let t4576 = t3274 * t1662 * t1045;
    let t4579 = t1103 * t347;
    let t4580 = t1071 * t1646;
    (t4568, t4572, t4576, t4579, t4580)
}
