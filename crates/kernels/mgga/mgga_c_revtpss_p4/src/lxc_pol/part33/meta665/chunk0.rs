//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2174/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2174<F: Float>(t27888: F, t27899: F, t27884: F, t27873: F, t97700: F, t98041: F, t22387: F, t22415: F, t28012: F, t7279: F, t7917: F, t94851: F, t94854: F, t94857: F, t98043: F, t98069: F, t98071: F, t98078: F, t98081: F) -> F {
    let t108431 = t27899 * t27888;
    let t108435 = t27884 * t27888;
    let t108438 = t97700 * t27873;
    let t108440 = t98041 * t27873;
    let t108443 = t98043 - F::cast_from(0.8673628188205199462e0_f64) * t7917 * t28012 + F::cast_from(0.24093411633903331839e-3_f64) * t94851 + F::cast_from(0.13170898365871023197e1_f64) * t7279 * t22415 + t98069 + t94854 + F::cast_from(0.14456046980341999104e-1_f64) * t108431 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t22387 + t98071 - F::cast_from(0.25702851531048074406e-1_f64) * t108435 + F::cast_from(0.48186823267806663678e-3_f64) * t94857 - F::cast_from(0.28912093960683998207e-1_f64) * t108438 + F::cast_from(0.51405703062096148813e-1_f64) * t108440 + F::cast_from(0.86736281882051994624e-1_f64) * t98078 - t98081;
    t108443
}
