//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2178/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2178<F: Float>(t7064: F, t99321: F, t25411: F, t99389: F, t93369: F, t93372: F, t93375: F, t93378: F, t93382: F, t93384: F, t99472: F, t99475: F, t99480: F, t99481: F, t99485: F, t99487: F) -> F {
    let t99491 = F::cast_from(0.25702851531048074406e-1_f64) * t7064 * t99321;
    let t99493 = F::cast_from(0.25702851531048074406e-1_f64) * t25411 * t99389;
    let t99494 = -t99472 + t99475 + F::cast_from(0.51405703062096148812e-1_f64) * t93369 + F::cast_from(0.45699670022203476294e-2_f64) * t93372 + F::cast_from(0.25702851531048074406e-1_f64) * t93375 - t99480 - F::cast_from(0.96373646535613327357e-2_f64) * t99481 - F::cast_from(0.68540937416128198418e-2_f64) * t93378 - t99485 - t99487 - F::cast_from(0.13009920719177044025e-2_f64) * t93382 - F::cast_from(0.19274729307122665471e-1_f64) * t93384 - t99491 + t99493;
    t99494
}
