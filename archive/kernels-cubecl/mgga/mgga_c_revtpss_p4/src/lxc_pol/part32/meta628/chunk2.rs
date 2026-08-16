//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2011/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2011<F: Float>(t105946: F, t7407: F, t106387: F, t30356: F, t686: F, t72: F, t25387: F, t103030: F, t103047: F, t103063: F, t103069: F, t103072: F, t231: F, t27199: F, t28348: F, t6016: F, t7070: F, t7076: F, t7398: F, t8007: F, t95722: F, t95727: F, t95732: F, t99303: F) -> (F, F) {
    let t110316 = t105946 * t7407;
    let t110318 = t106387 * t7407;
    let t110322 = t30356 * t72 * t686;
    let t110323 = t25387 * t110322;
    let t110330 = F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t7398 * t6016 * t231 + F::cast_from(0.17347256376410398924e1_f64) * t99303 * t8007 + F::cast_from(0.72280234901709995518e-2_f64) * t110316 - F::cast_from(0.12851425765524037203e-1_f64) * t110318 - F::cast_from(0.23131639038696784278e-2_f64) * t103030 - t103047 + F::cast_from(0.25702851531048074406e-1_f64) * t110323 + F::cast_from(0.19274729307122665471e-1_f64) * t95722 - F::cast_from(0.34270468708064099208e-2_f64) * t95727 - t95732 + F::cast_from(0.34270468708064099208e-1_f64) * t103063 - t103069 + t103072 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t28348;
    (t110322, t110330)
}
