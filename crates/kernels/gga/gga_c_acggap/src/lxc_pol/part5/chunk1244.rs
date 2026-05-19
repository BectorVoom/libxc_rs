//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1244/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1244<F: Float>(t1089: F, t1101: F, t1743: F, t384: F, t1096: F, t1165: F, t13664: F, t17484: F, t17501: F, t17503: F, t17505: F, t17507: F, t17509: F, t17511: F, t17513: F, t17521: F, t20764: F, t3396: F) -> F {
    let t22843 = t384 * t1089 * t1743 * t1101;
    let t22845 = F::cast_from(0.85748036236139473944e-3_f64) * t13664 - F::cast_from(0.16006300097412701803e-1_f64) * t17484 - F::cast_from(0.10289764348336736874e-1_f64) * t3396 * t1165 * t20764 * t1096 - F::cast_from(0.85748036236139473944e-3_f64) * t17501 + F::cast_from(0.16006300097412701803e-1_f64) * t17503 - F::cast_from(0.16006300097412701803e-1_f64) * t17505 - F::cast_from(0.12004725073059526352e-1_f64) * t17507 + F::cast_from(0.80031500487063509015e-2_f64) * t17509 - F::cast_from(0.24009450146119052704e-1_f64) * t17511 - F::cast_from(0.17149607247227894789e-1_f64) * t17513 + F::cast_from(0.80031500487063509015e-1_f64) * t17521 + F::cast_from(0.34299214494455789578e-2_f64) * t22843;
    t22845
}
