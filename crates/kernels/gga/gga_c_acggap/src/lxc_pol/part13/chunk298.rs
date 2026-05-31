//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 298/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk298<F: Float>(t1131: F, t336: F, t368: F, t1044: F, t1080: F, t1086: F, t1092: F, t1098: F, t1104: F, t1109: F, t1114: F, t1117: F, t1121: F, t127: F, t335: F, t367: F, t418: F) -> (F, F) {
    let t1133 = t336 * t368 * t1131;
    let t1136 = -F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t1044 + t127 * t1080 / F::cast_from(96.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t1086 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1092 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1098 + F::cast_from(0.17149607247227894789e-2_f64) * t1104 - F::cast_from(0.85748036236139473944e-3_f64) * t1109 + F::cast_from(0.85748036236139473944e-3_f64) * t1114 - t335 * t1117 / F::cast_from(48.0_f64) - t367 * t1121 / F::cast_from(48.0_f64) - t367 * t1133 / F::cast_from(96.0_f64);
    (t1133, t1136)
}
