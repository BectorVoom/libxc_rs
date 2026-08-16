//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 641/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk641<F: Float>(t1887: F, t5018: F, t1820: F, t1718: F, t401: F, t4367: F, t5002: F, t1714: F, t1642: F, t422: F, t1416: F, t657: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5019 = t5018 * t1887;
    let t5020 = t1820 * t5019;
    let t5021 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5020;
    let t5022 = t401 * t1718;
    let t5024 = t5002 * t4367;
    let t5025 = t1714 * t5024;
    let t5028 = t1642 * t422;
    let t5029 = t5028 * t1416;
    let t5030 = t1714 * t5029;
    let t5033 = t1642 * t4367;
    let t5034 = t657 * t5033;
    (t5019, t5021, t5022, t5024, t5025, t5028, t5029, t5030, t5033, t5034)
}
