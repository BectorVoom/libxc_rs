//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1137/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1137<F: Float>(t2173: F, t27856: F, t1087: F, t1774: F, t303: F, t26760: F, t4801: F, t1020: F, t4806: F, t7718: F, t4548: F, t4556: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27857 = t2173 * t27856;
    let t27859 = t1087 * t1774;
    let t27860 = t303 * t27859;
    let t27864 = t26760 * t4801;
    let t27865 = t1020 * t27864;
    let t27867 = t7718 * t4806;
    let t27868 = t1020 * t27867;
    let t27870 = t7718 * t4548;
    let t27871 = t1020 * t27870;
    let t27873 = t7718 * t4556;
    (t27857, t27859, t27860, t27864, t27865, t27867, t27868, t27870, t27871, t27873)
}
