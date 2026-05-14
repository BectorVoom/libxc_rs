//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1002/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1002<F: Float>(t26905: F, t3785: F, t1411: F, t26875: F, t5670: F, t19813: F, t5633: F, t3739: F, t8083: F, t14188: F, t8073: F, t3748: F, t8086: F, t5606: F, t5612: F, t1339: F) -> (F, F, F, F, F, F, F) {
    let t26906 = t3785 * t26905;
    let t26907 = t1411 * t26906;
    let t26910 = t5670 * t26875;
    let t26911 = t19813 * t26910;
    let t26912 = t5633 * t26911;
    let t26914 = t3739 * t8083;
    let t26916 = t14188 * t8073;
    let t26917 = t1411 * t26916;
    let t26919 = t3748 * t8086;
    let t26921 = t5606 * t5612;
    let t26922 = t1339 * t26921;
    (t26907, t26910, t26912, t26914, t26917, t26919, t26922)
}
