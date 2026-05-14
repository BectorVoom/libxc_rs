//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 677/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk677<F: Float>(t10968: F, t184: F, t3450: F, t582: F, t561: F, t3414: F, t5129: F, t587: F, t3454: F, t572: F, t3402: F, t4934: F, t1620: F, t3406: F, t5137: F, t639: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10969 = t10968 * t184;
    let t10972 = t582 * t3450;
    let t10973 = t561 * t10972;
    let t10992 = t5129 * t3414;
    let t10993 = t587 * t10992;
    let t11005 = t3454 * t572;
    let t11019 = t4934 * t3402;
    let t11020 = t1620 * t11019;
    let t11022 = t5137 * t3406;
    let t11023 = t639 * t11022;
    (t10969, t10972, t10973, t10992, t10993, t11005, t11019, t11020, t11022, t11023)
}
