//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1263/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1263<F: Float>(t6215: F, t952: F, t5950: F, t997: F, t3431: F, t6237: F, t3409: F, t6241: F, t3382: F, t1017: F, t1426: F, t1459: F, t1713: F, t17139: F, t176: F, t17921: F, t17926: F, t17928: F, t17930: F, t17932: F, t418: F, t5735: F, t8401: F) -> F {
    let t23309 = t952 * t6215;
    let t23311 = t997 * t5950;
    let t23314 = t3431 * t6237;
    let t23316 = t3409 * t6241;
    let t23318 = t3382 * t6241;
    let t23320 = -F::cast_from(0.17149607247227894789e-1_f64) * t17139 * t176 * t8401 * t5735 - F::cast_from(0.10289764348336736873e-1_f64) * t17921 - F::cast_from(0.68598428988911579156e-2_f64) * t17926 - F::cast_from(0.48018900292238105409e-1_f64) * t17928 - F::cast_from(0.32012600194825403606e-1_f64) * t17930 + F::cast_from(0.25724410870841842183e-1_f64) * t418 * t1426 * t1459 * t1713 * t1017 + F::cast_from(0.40015750243531754508e-2_f64) * t23309 - F::cast_from(0.80031500487063509015e-1_f64) * t23311 + F::cast_from(0.17149607247227894789e-2_f64) * t17932 - F::cast_from(0.80031500487063509015e-2_f64) * t23314 + F::cast_from(0.40015750243531754508e-2_f64) * t23316 - F::cast_from(0.85748036236139473944e-3_f64) * t23318;
    t23320
}
