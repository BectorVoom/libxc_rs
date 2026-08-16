//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 530/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk530<F: Float>(t2538: F, t4811: F, t2529: F, t2474: F, t5074: F, t2454: F, t642: F, t1871: F, t2507: F, t1333: F, t2534: F, t2510: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t6949 = t4811 * t2538;
    let t6951 = t4811 * t2529;
    let t6959 = t5074 * t2474;
    let t6965 = t2454 * t642;
    let t6973 = t2507 * t1871;
    let t6974 = t6973 * sigma2;
    let t6990 = t1333 * t2534;
    let t6992 = t1333 * t2510;
    (t6949, t6951, t6959, t6965, t6973, t6974, t6990, t6992)
}
