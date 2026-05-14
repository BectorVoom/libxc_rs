//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 960/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk960<F: Float>(t16905: F, t498: F, t12147: F, t5722: F, t1368: F, t5705: F, t3970: F, t5441: F, t12140: F, t5427: F, t1930: F, t3967: F, t1377: F, t3977: F, t3754: F, t1369: F) -> (F, F, F, F, F, F, F, F) {
    let t16906 = t16905 * t498;
    let t16923 = t12147 * t5722;
    let t16925 = t1368 * t16923 / 432.0;
    let t16933 = t12147 * t5705;
    let t16935 = t1368 * t16933 / 432.0;
    let t16937 = t3970 * t498;
    let t16938 = t16937 * t5441;
    let t16940 = t1368 * t16938 / 216.0;
    let t16941 = t12140 * t498;
    let t16942 = t16941 * t5427;
    let t16944 = t1368 * t16942 / 324.0;
    let t16954 = t1930 * t3967;
    let t16962 = t3977 * t1377;
    let t16963 = t16962 * t3754;
    let t16968 = t1369 * t1377;
    (t16906, t16925, t16935, t16940, t16944, t16954, t16963, t16968)
}
