//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1289/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1289<F: Float>(t17000: F, t1869: F, t33017: F, t32942: F, t34118: F, t32990: F, t17163: F, t34132: F, t9664: F, t16604: F, t1799: F, t9679: F, t16609: F, t5054: F, t34212: F, t5074: F) -> (F, F, F, F, F, F, F) {
    let t116361 = t1869 * t33017 * t17000;
    let t116368 = 0.23148148148148148148e-2 * t32942 * t34118;
    let t116370 = 0.23148148148148148148e-2 * t32990 * t34118;
    let t116372 = t9664 * t17163 * t34132;
    let t116375 = t1799 * t9679 * t16604;
    let t116378 = t5054 * t9679 * t16609;
    let t116380 = t5074 * t34212;
    (t116361, t116368, t116370, t116372, t116375, t116378, t116380)
}
