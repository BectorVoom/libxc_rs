//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 784/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk784<F: Float>(t132: F, t2808: F, t2810: F, t721: F, t200: F, t220: F, t328: F, t123: F, t759: F, t762: F, t2604: F, t704: F, t2773: F, t286: F, t680: F, t800: F) -> (F, F, F, F, F) {
    let t11578 = 0.68734380377411894876e1 * t721 * t132 * t2808 * t2810;
    let t11582 = 0.22161481481481481481e0 * t721 * t328 * t200 * t220;
    let t11586 = 0.28493333333333333333e0 * t721 * t123 * t759 * t762;
    let t11591 = t704 * t2604;
    let t11596 = 0.62337092780453269531e3 * t286 * t2773 * t680 * t800;
    (t11578, t11582, t11586, t11591, t11596)
}
