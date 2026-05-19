//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1177/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1177<F: Float>(t1131: F, t1150: F, t1180: F, t1181: F, t1532: F, t16498: F, t16500: F, t16510: F, t16524: F, t1889: F, t21331: F, t21338: F, t21340: F, t21342: F, t21348: F, t335: F, t372: F, t4578: F, t4593: F, t5688: F, t960: F) -> F {
    let t21351 = t1150 * t4593 * t4578 / F::new(4.0) + t335 * t960 * t5688 * t372 / F::new(24.0) + t335 * t960 * t1889 * t1131 / F::new(48.0) - F::new(7.0) / F::new(36.0) * t21331 + F::cast_from(0.40015750243531754508e-2_f64) * t16498 + F::cast_from(0.16006300097412701803e-1_f64) * t16500 + F::cast_from(0.85748036236139473944e-3_f64) * t21338 - F::cast_from(0.34299214494455789578e-2_f64) * t21340 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1181 * t1532 * t21342 - F::cast_from(0.10289764348336736873e-1_f64) * t16510 - F::cast_from(0.32012600194825403606e-1_f64) * t21348 + F::cast_from(0.17149607247227894789e-2_f64) * t16524;
    t21351
}
