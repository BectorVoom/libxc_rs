//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2034/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2034<F: Float>(t106494: F, t26425: F, t102888: F, t106490: F, t106498: F, t106502: F, t106520: F, t106528: F, t106572: F, t106583: F, t106602: F, t1940: F, t2403: F, t27166: F, t27376: F, t27387: F, t27391: F, t27395: F, t28291: F, t28460: F, t30420: F, t605: F, t7432: F, t8020: F) -> (F, F) {
    let t110717 = F::new(6.0) * t26425 * t106494;
    let t110745 = F::new(6.0) * t26425 * t106502 - F::new(3.0) * t26425 * t106490 + t110717 + F::new(3.0) * t28291 * t106498 - F::new(3.0) * t102888 * t27376 - t1940 * t28460 * t27387 - F::new(3.0) * t26425 * t106520 - t1940 * t28460 * t27391 - F::new(3.0) * t102888 * t27166 - t1940 * t7432 * t106583 + F::new(3.0) * t2403 * t8020 * t27395 + F::new(6.0) * t28291 * t106572 + t1940 * t30420 * t605 / F::new(2.0) - t1940 * t7432 * t106602 / F::new(2.0) - F::new(3.0) * t26425 * t106528;
    (t110717, t110745)
}
