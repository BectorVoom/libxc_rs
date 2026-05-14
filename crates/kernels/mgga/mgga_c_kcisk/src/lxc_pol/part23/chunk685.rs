//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 685/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk685<F: Float>(t1254: F, t6078: F, t2128: F, t4083: F, t1253: F, t4011: F, t4087: F, t6020: F, t6023: F, t6026: F, t6030: F, t2133: F, t45: F, t2141: F, t4100: F, t1273: F, t1275: F) -> (F, F, F, F, F, F, F) {
    let t6079 = t6078 * t1254;
    let t6082 = t2128 * t4083;
    let t6083 = t6082 * t1253;
    let t6091 = t4087 + 0.30902777777777777778e-2 * t4011 + 0.30902777777777777778e-2 * t6020 - 0.61805555555555555555e-2 * t6023 + 0.18541666666666666667e-1 * t6026 - 0.18541666666666666667e-1 * t6030;
    let t6095 = t45 * t2133;
    let t6100 = t4100 * t2141;
    let t6101 = t1275 * t1273;
    (t6079, t6082, t6083, t6091, t6095, t6100, t6101)
}
