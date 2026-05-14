//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 837/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk837<F: Float>(t16937: F, t5441: F, t1368: F, t12140: F, t498: F, t5427: F, t2645: F, t5721: F, t3984: F, t1938: F, t3754: F, t2642: F, t12194: F, t1930: F, t3967: F, t4007: F, t5726: F) -> (F, F, F, F, F, F, F) {
    let t16938 = t16937 * t5441;
    let t16940 = t1368 * t16938 / 216.0;
    let t16941 = t12140 * t498;
    let t16942 = t16941 * t5427;
    let t16944 = t1368 * t16942 / 324.0;
    let t16945 = t5721 * t2645;
    let t16946 = t3984 * t16945;
    let t16949 = t1938 * t3754;
    let t16950 = t16949 * t2642;
    let t16951 = t12194 * t16950;
    let t16954 = t1930 * t3967;
    let t16958 = t5726 * t4007;
    (t16940, t16941, t16944, t16946, t16951, t16954, t16958)
}
