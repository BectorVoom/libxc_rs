//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 952/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk952<F: Float>(t16034: F, t1882: F, t706: F, t1824: F, t6791: F, t7055: F, t3521: F, t7052: F, t7057: F, t4652: F, t7050: F, t4629: F, t16076: F, t1887: F, t2522: F, t3517: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16854 = t1882 * t16034;
    let t16855 = t706 * t16854;
    let t16858 = t6791 * t1824;
    let t16859 = t7055 * t16858;
    let t16863 = 0.19711289e-2 * t3521 * t7052;
    let t16865 = 0.26281718666666666666e-2 * t3521 * t7057;
    let t16866 = t7050 * t4652;
    let t16867 = t4629 * t16866;
    let t16872 = t1887 * t16076;
    let t16873 = t706 * t16872;
    let t16879 = t3517 * t2522;
    (t16854, t16855, t16858, t16859, t16863, t16865, t16866, t16867, t16872, t16873, t16879)
}
