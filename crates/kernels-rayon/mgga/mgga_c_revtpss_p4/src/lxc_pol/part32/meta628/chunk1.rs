//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2010/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2010(t30400: f64, t689: f64, t25431: f64, t25411: f64, t103001: f64, t103007: f64, t103009: f64, t103017: f64, t103023: f64, t18615: f64, t2061: f64, t231: f64, t25383: f64, t28436: f64, t30406: f64, t4423: f64, t7070: f64, t7076: f64, t7997: f64, t95607: f64, t95620: f64, t95629: f64, t95632: f64, t99191: f64) -> f64 {
    let t110288 = t30400 * t689;
    let t110289 = t25431 * t110288;
    let t110291 = t25411 * t110288;
    let t110306 = -t95607 - 0.3427046870806409921e-2_f64 * t103001 + 0.73171657588172351096e-2_f64 * t95620 + t103007 + 0.91399340044406952588e-2_f64 * t103009 - 0.34270468708064099208e-1_f64 * t95629 - 0.17347256376410398924e1_f64 * t99191 * t28436 + t95632 - 0.14456046980341999104e-1_f64 * t110289 + 0.25702851531048074406e-1_f64 * t110291 - 0.4818682326780666368e-3_f64 * t103017 + t103023 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t2061 * t18615 * t231 + 0.8673628188205199462e0_f64 * t7070 * t7076 * t7997 * t4423 * t231 + 0.4336814094102599731e0_f64 * t25383 * t30406;
    t110306
}
