//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1274/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1274<F: Float>(t109398: F, t9536: F, t32339: F, t32342: F, t3913: F, t491: F, t1333: F, t32062: F, t1310: F, t1588: F, t3951: F, t1413: F, t3907: F, t32119: F, t32123: F, t32208: F, t3748: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109934 = t9536 * t109398;
    let t109941 = t32339 * t32342;
    let t109952 = t491 * t3913;
    let t109956 = t1333 * t32062;
    let t109963 = t1310 * t3951 * t1588;
    let t110025 = t3907 * t1413;
    let t110029 = t1333 * t32119;
    let t110037 = t1333 * t32123;
    let t110064 = t3748 * t32208;
    (t109934, t109941, t109952, t109956, t109963, t110025, t110029, t110037, t110064)
}
