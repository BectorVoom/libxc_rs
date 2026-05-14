//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 677/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk677<F: Float>(t17899: F, t2394: F, t1127: F, t3751: F, t680: F, t1096: F, t3817: F, t122: F, t237: F, t3758: F, t1113: F, t689: F, t2427: F, t25: F, t677: F, t200: F) -> (F, F, F, F, F, F, F, F) {
    let t17933 = t2394 * t17899;
    let t17936 = t3751 * t1127;
    let t17937 = t680 * t17936;
    let t17941 = t680 * t1096 * t3817;
    let t17944 = t237 * t122;
    let t17945 = t3758 * t17944;
    let t17946 = t689 * t1113;
    let t17950 = t689 * t1127;
    let t17957 = t2427 * t25;
    let t17958 = t677 * t17957;
    let t17959 = t200 * t1127;
    (t17933, t17937, t17941, t17945, t17946, t17950, t17958, t17959)
}
