//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 557/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk557<F: Float>(t7699: F, t4685: F, t681: F, t4616: F, t664: F) -> (F, F, F) {
    let t7700 = 0.19957069503106347607e-1 * t7699;
    let t7701 = t4685 * t681;
    let t7702 = 0.14967802127329760705e-1 * t7701;
    let t7703 = t4616 * t664;
    (t7700, t7702, t7703)
}
