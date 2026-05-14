//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1048/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1048<F: Float>(t2059: F, t6581: F, t4400: F, t1312: F, t3973: F, t8331: F, t1580: F, t14978: F, t27817: F, t25342: F, t4391: F, t25413: F, t4406: F, t6187: F, t25906: F, t41: F) -> (F, F, F, F, F, F, F) {
    let t27854 = t2059 * t6581;
    let t27855 = t4400 * t27854;
    let t27856 = t1312 * t27855;
    let t27861 = t3973 * t8331;
    let t27862 = t1580 * t27861;
    let t27864 = t14978 * t27817;
    let t27865 = t1312 * t27864;
    let t27868 = t4391 * t25342;
    let t27869 = t1312 * t27868;
    let t27872 = t4406 * t25413;
    let t27873 = t6187 * t27872;
    let t27876 = t25906 * t41;
    (t27854, t27856, t27862, t27865, t27869, t27873, t27876)
}
