//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 655/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk655<F: Float>(t2474: F, t5074: F, t1894: F, t6697: F, t1873: F, t1869: F, t2454: F, t642: F, t1757: F, t1800: F, t2537: F, t4581: F, t1871: F, t2507: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6959 = t5074 * t2474;
    let t6961 = t6697 * t1894;
    let t6962 = t1873 * t6961;
    let t6963 = t1869 * t6962;
    let t6965 = t2454 * t642;
    let t6966 = t6965 * t1757;
    let t6967 = t1800 * t6966;
    let t6968 = t1869 * t6967;
    let t6970 = t4581 * t2537;
    let t6971 = t1869 * t6970;
    let t6973 = t2507 * t1871;
    (t6959, t6961, t6962, t6963, t6965, t6966, t6967, t6968, t6970, t6971, t6973)
}
