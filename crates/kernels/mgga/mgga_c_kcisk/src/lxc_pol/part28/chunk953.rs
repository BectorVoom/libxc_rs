//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 953/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk953<F: Float>(t22488: F, t7012: F, t1648: F, t4604: F, t8518: F, t1824: F, t4609: F, t1849: F, t7718: F, t1060: F) -> (F, F, F, F) {
    let t22489 = t7012 * t22488;
    let t22493 = t4604 * t8518 * t1648;
    let t22497 = t4609 * t8518 * t1824;
    let t22500 = t1849 * t7718;
    let t22501 = t22500 * t1060;
    (t22489, t22493, t22497, t22501)
}
