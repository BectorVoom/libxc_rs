//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 911/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk911<F: Float>(t12430: F, t10945: F, t10948: F, t10957: F, t10965: F, t10970: F, t10974: F, t10983: F, t10991: F, t10996: F, t11616: F, t11545: F, t3579: F, t1103: F, t3128: F, t1053: F, t1102: F) -> (F, F, F, F, F) {
    let t12431 = 5.0 / 16.0 * t12430;
    let t12432 = t10945 + t10948 + t10957 - t10965 + t10970 + t10974 - t10983 - 0.81300399444200075504e-3 * t11616 + t10991 + t10996 + t12431;
    let t12433 = t3579 * t11545;
    let t12434 = 5.0 / 8.0 * t12433;
    let t12435 = t1103 * t3128;
    let t12437 = t1102 * t1053 * t12435;
    (t12431, t12432, t12434, t12435, t12437)
}
