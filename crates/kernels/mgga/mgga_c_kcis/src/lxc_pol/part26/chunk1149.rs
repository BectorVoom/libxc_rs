//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1149/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1149<F: Float>(t27394: F, t7080: F, t6176: F, t2237: F, t28397: F, t28415: F, t28424: F, t28427: F, t28544: F, t29284: F, t29300: F, t29305: F, t29308: F, t29311: F, t29314: F, t29324: F, t7898: F, t7908: F, t8144: F, t8148: F, t8159: F) -> (F, F, F) {
    let t29331 = t27394 * t7080;
    let t29332 = t6176 * t29331;
    let t29335 = F::new(0.18550940104166666667e-3) * t28397 * t8148 + F::new(0.92754700520833333333e-4) * t7898 * t29300 + F::new(0.22109259259259259258e-2) * t28415 - F::new(0.88437037037037037034e-2) * t29305 + F::new(0.16581944444444444444e-2) * t29308 - F::new(0.55273148148148148147e-3) * t29311 + F::new(0.46336805555555555556e-3) * t7908 * t29314 + F::new(0.46336805555555555556e-3) * t7908 * t29284 + F::new(0.46336805555555555556e-3) * t28424 + F::new(0.61836467013888888889e-4) * t28427 + F::new(0.13901041666666666667e-2) * t8144 * t8159 - F::new(0.2782641015625e-3) * t7898 * t29324 - F::new(0.4946917361111111111e-3) * t28544 * t8148 - F::new(0.13901041666666666667e-2) * t2237 * t29324 - F::new(0.13901041666666666667e-2) * t2237 * t29332;
    (t29331, t29332, t29335)
}
