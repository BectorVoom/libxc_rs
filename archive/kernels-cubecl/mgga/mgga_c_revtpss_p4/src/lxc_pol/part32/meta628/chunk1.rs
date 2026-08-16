//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2010/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2010<F: Float>(t30400: F, t689: F, t25431: F, t25411: F, t103001: F, t103007: F, t103009: F, t103017: F, t103023: F, t18615: F, t2061: F, t231: F, t25383: F, t28436: F, t30406: F, t4423: F, t7070: F, t7076: F, t7997: F, t95607: F, t95620: F, t95629: F, t95632: F, t99191: F) -> F {
    let t110288 = t30400 * t689;
    let t110289 = t25431 * t110288;
    let t110291 = t25411 * t110288;
    let t110306 = -t95607 - F::cast_from(0.3427046870806409921e-2_f64) * t103001 + F::cast_from(0.73171657588172351096e-2_f64) * t95620 + t103007 + F::cast_from(0.91399340044406952588e-2_f64) * t103009 - F::cast_from(0.34270468708064099208e-1_f64) * t95629 - F::cast_from(0.17347256376410398924e1_f64) * t99191 * t28436 + t95632 - F::cast_from(0.14456046980341999104e-1_f64) * t110289 + F::cast_from(0.25702851531048074406e-1_f64) * t110291 - F::cast_from(0.4818682326780666368e-3_f64) * t103017 + t103023 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t2061 * t18615 * t231 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t7997 * t4423 * t231 + F::cast_from(0.4336814094102599731e0_f64) * t25383 * t30406;
    t110306
}
