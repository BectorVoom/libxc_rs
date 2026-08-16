//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2202/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2202(t27888: f64, t27899: f64, t27884: f64, t27873: f64, t97700: f64, t98041: f64, t22387: f64, t22415: f64, t28012: f64, t7279: f64, t7917: f64, t94851: f64, t94854: f64, t94857: f64, t98043: f64, t98069: f64, t98071: f64, t98078: f64, t98081: f64) -> f64 {
    let t108431 = t27899 * t27888;
    let t108435 = t27884 * t27888;
    let t108438 = t97700 * t27873;
    let t108440 = t98041 * t27873;
    let t108443 = t98043 - 0.8673628188205199462e0_f64 * t7917 * t28012 + 0.24093411633903331839e-3_f64 * t94851 + 0.13170898365871023197e1_f64 * t7279 * t22415 + t98069 + t94854 + 0.14456046980341999104e-1_f64 * t108431 - 0.65854491829355115987e0_f64 * t7279 * t22387 + t98071 - 0.25702851531048074406e-1_f64 * t108435 + 0.48186823267806663678e-3_f64 * t94857 - 0.28912093960683998207e-1_f64 * t108438 + 0.51405703062096148813e-1_f64 * t108440 + 0.86736281882051994624e-1_f64 * t98078 - t98081;
    t108443
}
