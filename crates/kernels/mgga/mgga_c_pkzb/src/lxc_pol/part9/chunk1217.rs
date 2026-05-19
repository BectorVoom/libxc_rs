//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1217/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1217<F: Float>(t1856: F, t1899: F, t2783: F, t2786: F, t5737: F, t5802: F, t1084: F, t5776: F, t1083: F, t17577: F, t17579: F, t5585: F, t7411: F) -> (F, F, F, F, F) {
    let t21236 = F::new(18.0) * t1899 * t2783 * t1856;
    let t21239 = F::cast_from(0.57895126195293126241e3_f64) * t5802 * t2786 * t5737;
    let t21251 = F::new(24.0) * t5776 * t1084 * t5737;
    let t21255 = F::cast_from(0.24955700379505800916e5_f64) * t17577 * t1083 * t17579 * t5737;
    let t21257 = F::cast_from(0.48245938496077605201e2_f64) * t7411 * t5585;
    (t21236, t21239, t21251, t21255, t21257)
}
