//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2011/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2011(t105946: f64, t7407: f64, t106387: f64, t30356: f64, t686: f64, t72: f64, t25387: f64, t103030: f64, t103047: f64, t103063: f64, t103069: f64, t103072: f64, t231: f64, t27199: f64, t28348: f64, t6016: f64, t7070: f64, t7076: f64, t7398: f64, t8007: f64, t95722: f64, t95727: f64, t95732: f64, t99303: f64) -> (f64, f64) {
    let t110316 = t105946 * t7407;
    let t110318 = t106387 * t7407;
    let t110322 = t30356 * t72 * t686;
    let t110323 = t25387 * t110322;
    let t110330 = 0.4336814094102599731e0_f64 * t7070 * t7076 * t7398 * t6016 * t231 + 0.17347256376410398924e1_f64 * t99303 * t8007 + 0.72280234901709995518e-2_f64 * t110316 - 0.12851425765524037203e-1_f64 * t110318 - 0.23131639038696784278e-2_f64 * t103030 - t103047 + 0.25702851531048074406e-1_f64 * t110323 + 0.19274729307122665471e-1_f64 * t95722 - 0.34270468708064099208e-2_f64 * t95727 - t95732 + 0.34270468708064099208e-1_f64 * t103063 - t103069 + t103072 + 0.17347256376410398924e1_f64 * t27199 * t28348;
    (t110322, t110330)
}
