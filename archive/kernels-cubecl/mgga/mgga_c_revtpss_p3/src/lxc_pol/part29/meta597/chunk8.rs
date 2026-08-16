//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2023/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2023<F: Float>(t103438: F, t25372: F, t98801: F, t25386: F, t2471: F, t28373: F, t103352: F, t103422: F, t103424: F, t103432: F, t103435: F, t103437: F, t1956: F, t1957: F, t233: F, t25383: F, t25391: F, t25394: F, t26493: F, t26550: F, t27199: F, t27353: F, t28411: F, t51436: F, t95872: F, t95876: F) -> F {
    let t103441 = F::cast_from(0.14456046980341999104e-1_f64) * t25372 * t103438 * t98801;
    let t103444 = F::cast_from(0.25702851531048074406e-1_f64) * t25386 * t103438 * t98801;
    let t103449 = t28373 * t2471;
    let t103451 = -F::cast_from(0.4336814094102599731e0_f64) * t1956 * t1957 * t233 * t103352 - F::cast_from(0.52041769129231196772e1_f64) * t25383 * t28411 + F::cast_from(0.17135234354032049604e-1_f64) * t103422 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t103424 * t25394 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t26550 * t51436 - F::cast_from(0.34270468708064099208e-1_f64) * t103432 + t103435 - t103437 - t103441 + t103444 + F::cast_from(0.14456046980341999104e-1_f64) * t95872 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t26493 + F::cast_from(0.72280234901709995518e-2_f64) * t95876 - F::cast_from(0.13009920719177044025e-1_f64) * t103449;
    t103451
}
