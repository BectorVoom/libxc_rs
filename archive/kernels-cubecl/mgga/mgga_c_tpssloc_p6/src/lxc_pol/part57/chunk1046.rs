//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1046/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1046<F: Float>(t128293: F, t128388: F, t128433: F, t128469: F, t128514: F, t128553: F, t128593: F, t128973: F, t27254: F, t7467: F, t100996: F, t1873: F) -> (F, F, F) {
    let t128976 = t128293 + t128388 + t128433 + t128469 + t128514 + t128553 + t128593 + t128973;
    let t128984 = F::cast_from(27.0_f64) * t27254 * t7467;
    let t128988 = F::cast_from(0.135e2_f64) * t100996 * t1873;
    (t128976, t128984, t128988)
}
