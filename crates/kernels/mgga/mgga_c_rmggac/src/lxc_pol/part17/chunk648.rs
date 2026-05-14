//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 648/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk648<F: Float>(t10053: F, t4044: F, t2344: F, t8659: F, t2329: F, t8365: F, t209: F, t605: F, t615: F, t236: F, t1971: F, t7453: F, t618: F, t7231: F, t1970: F, t551: F) -> (F, F, F, F, F, F, F, F) {
    let t10054 = t4044 * t10053;
    let t10055 = 0.17961362552795712846e0 * t10054;
    let t10056 = t8659 * t2344;
    let t10057 = 0.20455996240684006296e-1 * t10056;
    let t10058 = t8365 * t2329;
    let t10059 = 0.27274661654245341728e-1 * t10058;
    let t10064 = t615 * t605 * t209;
    let t10065 = t236 * t10064;
    let t10066 = t1971 * t10065;
    let t10067 = t7453 * t10066;
    let t10068 = 0.1064114997332445985e-4 * t10067;
    let t10070 = t618 * t605 * t209;
    let t10071 = t236 * t10070;
    let t10072 = t7231 * t10071;
    let t10073 = t1970 * t10072;
    let t10074 = 0.85129199786595678796e-5 * t10073;
    let t10076 = t551 * t605 * t209;
    (t10055, t10057, t10059, t10066, t10068, t10072, t10074, t10076)
}
