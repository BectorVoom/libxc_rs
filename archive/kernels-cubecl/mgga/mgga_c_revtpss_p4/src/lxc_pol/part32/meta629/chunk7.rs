//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2026/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2026<F: Float>(t28368: F, t99404: F, t98849: F, t30405: F, t689: F, t25431: F, t25411: F, t103400: F, t103404: F, t103422: F, t103424: F, t106290: F, t106410: F, t25391: F, t26550: F, t27199: F, t27349: F, t27353: F, t28385: F, t30337: F, t62624: F, t8012: F, t92917: F, t99303: F) -> F {
    let t110525 = t99404 * t28368;
    let t110527 = t98849 * t28368;
    let t110541 = t30405 * t689;
    let t110542 = t25431 * t110541;
    let t110544 = t25411 * t110541;
    let t110551 = -F::cast_from(0.14634331517634470219e-1_f64) * t103400 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t26550 * t106410 - F::cast_from(0.14456046980341999104e-1_f64) * t110525 + F::cast_from(0.25702851531048074406e-1_f64) * t110527 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t26550 * t62624 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t26550 * t106290 - F::cast_from(0.34270468708064099208e-1_f64) * t103404 + F::cast_from(0.8673628188205199462e0_f64) * t99303 * t8012 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t28385 + F::cast_from(0.34270468708064099208e-1_f64) * t103422 - F::cast_from(0.72280234901709995518e-2_f64) * t110542 + F::cast_from(0.12851425765524037203e-1_f64) * t110544 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t103424 * t27349 - F::cast_from(0.17347256376410398924e1_f64) * t92917 * t30337;
    t110551
}
