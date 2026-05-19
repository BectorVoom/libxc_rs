//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1294/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1294<F: Float>(t3409: F, t5986: F, t1163: F, t1181: F, t1894: F, t4210: F, t1173: F, t13286: F, t13287: F, t14339: F, t18588: F, t18605: F, t18607: F, t18611: F, t18616: F, t18620: F, t18622: F, t23445: F, t23718: F, t525: F, t530: F) -> F {
    let t24026 = t3409 * t5986;
    let t24042 = t1163 * t1181 * t1894 * t4210;
    let t24048 = F::cast_from(0.24009450146119052704e-1_f64) * t24026 + F::cast_from(0.17149607247227894789e-1_f64) * t18588 - F::cast_from(0.13719685797782315831e-1_f64) * t13286 * t13287 * t525 * t23718 + F::cast_from(0.68598428988911579156e-2_f64) * t1173 * t1181 * t530 * t23445 + F::cast_from(0.68598428988911579156e-2_f64) * t18605 + F::cast_from(0.32012600194825403606e-1_f64) * t18607 + F::cast_from(0.17149607247227894789e-2_f64) * t18611 + F::cast_from(0.85748036236139473944e-3_f64) * t24042 + F::cast_from(0.34299214494455789578e-2_f64) * t18616 - F::cast_from(0.17149607247227894789e-2_f64) * t18620 - F::cast_from(0.16006300097412701803e-1_f64) * t18622 - F::cast_from(0.32012600194825403606e-1_f64) * t14339;
    t24048
}
