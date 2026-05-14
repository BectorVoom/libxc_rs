//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 786/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk786<F: Float>(t16937: F, t5441: F, t1368: F, t12140: F, t498: F, t5427: F, t1930: F, t3967: F, t1377: F, t3977: F, t3754: F, t1369: F, t1444: F, t25: F, t5733: F, t493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16938 = t16937 * t5441;
    let t16940 = t1368 * t16938 / 216.0;
    let t16941 = t12140 * t498;
    let t16942 = t16941 * t5427;
    let t16944 = t1368 * t16942 / 324.0;
    let t16954 = t1930 * t3967;
    let t16962 = t3977 * t1377;
    let t16963 = t16962 * t3754;
    let t16968 = t1369 * t1377;
    let t16969 = t16968 * t1444;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / 144.0;
    (t16940, t16941, t16944, t16954, t16962, t16963, t16968, t16969, t16981)
}
