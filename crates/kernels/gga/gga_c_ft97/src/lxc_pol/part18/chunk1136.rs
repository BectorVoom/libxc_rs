//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1136/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1136<F: Float>(t23578: F, t8392: F, t23583: F, t23455: F, t50249: F, t23571: F, t50235: F, t1882: F, t23587: F, t1366: F, t3281: F, t23568: F, t23471: F, t23978: F, t23957: F, t23940: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t95827 = t8392 * t23578;
    let t95829 = t8392 * t23583;
    let t95837 = t50249 * t23455;
    let t95842 = t50235 * t23571;
    let t95849 = t1882 * t23587;
    let t95859 = 28.0 / 81.0 * t3281 * t1366;
    let t95890 = t8392 * t23568;
    let t95898 = t8392 * t23471;
    let t95919 = t1882 * t23978;
    let t95936 = t1882 * t23957;
    let t95938 = t1882 * t23940;
    (t95827, t95829, t95837, t95842, t95849, t95859, t95890, t95898, t95919, t95936, t95938)
}
