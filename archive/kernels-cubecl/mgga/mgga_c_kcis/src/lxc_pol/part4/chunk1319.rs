//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1319/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1319<F: Float>(t3970: F, t498: F, t5441: F, t1368: F, t12140: F, t5427: F, t2645: F, t5721: F, t3984: F, t1938: F, t3754: F, t2642: F) -> (F, F, F, F) {
    let t16937 = t3970 * t498;
    let t16938 = t16937 * t5441;
    let t16940 = t1368 * t16938 / F::cast_from(216.0_f64);
    let t16941 = t12140 * t498;
    let t16942 = t16941 * t5427;
    let t16944 = t1368 * t16942 / F::cast_from(324.0_f64);
    let t16945 = t5721 * t2645;
    let t16946 = t3984 * t16945;
    let t16949 = t1938 * t3754;
    let t16950 = t16949 * t2642;
    (t16940, t16944, t16946, t16950)
}
