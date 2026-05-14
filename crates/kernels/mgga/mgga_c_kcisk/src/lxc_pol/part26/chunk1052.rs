//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1052/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1052<F: Float>(t1597: F, t27934: F, t14665: F, t1557: F, t1598: F, t21408: F, t21409: F, t26799: F, t26803: F, t26807: F, t26811: F, t26816: F, t26820: F, t26825: F, t26828: F, t26831: F, t26835: F, t26841: F, t26843: F, t27694: F, t4324: F, t6426: F, t6588: F, t8289: F) -> (F,) {
    let t27935 = t27934 * t1597;
    let t27944 = -t21408 - 0.11607361111111111111e-2 * t26799 - 0.38691203703703703703e-3 * t26803 + 0.61905925925925925925e-2 * t26807 - 0.15476481481481481481e-2 * t26811 + 0.46429444444444444444e-2 * t26816 - 0.38691203703703703703e-2 * t26820 + 0.30952962962962962962e-2 * t26825 + t21409 - 0.34822083333333333332e-2 * t26828 - 0.193e0 * t27694 * t1598 + 0.11607361111111111111e-2 * t26831 + 0.11607361111111111111e-2 * t26835 + t14665 - 0.193e0 * t1557 * t27935 + 0.193e0 * t4324 * t8289 - 0.15476481481481481481e-2 * t26841 - 0.386e0 * t6426 * t6588 - 0.23214722222222222222e-2 * t26843;
    (t27944,)
}
