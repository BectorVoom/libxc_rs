//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 921/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk921<F: Float>(t4048: F, t424: F, t116: F, t14873: F, t3116: F, t4687: F, t102: F, t5390: F, t8959: F, t4939: F, t8676: F, t19765: F, t3141: F, t20500: F, t3712: F, t1: F) -> (F, F, F, F, F, F, F, F) {
    let t25530 = t424 * t4048;
    let t25708 = t116 * t14873;
    let t25756 = t3116 * t4687;
    let t25813 = t8959 * t5390 * t102;
    let t25842 = t8676 * t4939;
    let t25871 = t3141 * t19765;
    let t25876 = t3712 * t20500;
    let t25953 = t424 * t1;
    (t25530, t25708, t25756, t25813, t25842, t25871, t25876, t25953)
}
