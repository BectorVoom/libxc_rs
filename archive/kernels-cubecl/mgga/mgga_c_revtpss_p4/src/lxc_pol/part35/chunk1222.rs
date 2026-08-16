//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1222/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1222<F: Float>(t106024: F, t106030: F, t106033: F, t106037: F, t106040: F, t106042: F, t106048: F, t106050: F, t95671: F, t98976: F, t98979: F, t99002: F, t99009: F, t99013: F) -> F {
    let t115687 = -F::cast_from(0.24009450146119052704e0_f64) * t106024 - F::cast_from(0.2168591159877823526e-3_f64) * t98976 + F::cast_from(0.30492001685571196935e-4_f64) * t98979 + F::cast_from(0.16262400898971305032e-2_f64) * t99002 - t95671 - F::cast_from(0.27210710165601593065e0_f64) * t99009 - F::cast_from(0.17149607247227894789e-3_f64) * t106030 + F::cast_from(0.85748036236139473944e-4_f64) * t106033 + F::cast_from(0.65049603595885220128e-2_f64) * t99013 - F::cast_from(0.6098400337114239387e-3_f64) * t106037 + F::cast_from(0.85748036236139473944e-4_f64) * t106040 + F::cast_from(0.12004725073059526352e-1_f64) * t106042 - F::cast_from(0.15246000842785598468e-3_f64) * t106048 + F::cast_from(0.30492001685571196935e-3_f64) * t106050;
    t115687
}
