//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1254/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1254<F: Float>(t1106: F, t1181: F, t1899: F, t3391: F, t1165: F, t17710: F, t17718: F, t17721: F, t17725: F, t23063: F, t23065: F, t23068: F, t23070: F, t23077: F, t23081: F, t3396: F, t4665: F, t6138: F) -> F {
    let t23088 = t3391 * t1181 * t1899 * t1106;
    let t23090 = F::cast_from(0.16006300097412701803e-1_f64) * t23063 + F::cast_from(0.16006300097412701803e-1_f64) * t23065 - F::cast_from(0.68598428988911579156e-2_f64) * t17710 - F::cast_from(0.32012600194825403606e-1_f64) * t23068 - F::cast_from(0.20579528696673473746e-1_f64) * t23070 - F::cast_from(0.10289764348336736873e-1_f64) * t3396 * t1165 * t6138 * t4665 + F::cast_from(0.34299214494455789578e-2_f64) * t23077 + F::cast_from(0.34299214494455789578e-2_f64) * t23081 + F::cast_from(0.68598428988911579156e-2_f64) * t17718 + F::cast_from(0.34299214494455789578e-2_f64) * t17721 + F::cast_from(0.34299214494455789578e-2_f64) * t17725 + F::cast_from(0.17149607247227894789e-2_f64) * t23088;
    t23090
}
