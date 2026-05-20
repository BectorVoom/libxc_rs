//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1999/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1999<F: Float>(t102993: F, t25431: F, t2439: F, t8011: F, t93170: F, t28347: F, t686: F, t72: F, t25387: F, t102981: F, t102984: F, t102988: F, t25383: F, t2772: F, t28348: F, t28394: F, t95594: F, t95598: F, t95604: F, t95607: F, t95613: F, t95620: F) -> (F, F, F) {
    let t102994 = t25431 * t102993;
    let t103000 = t8011 * t2439;
    let t103001 = t93170 * t103000;
    let t103005 = t28347 * t72 * t686;
    let t103007 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t103005;
    let t103008 = -F::cast_from(0.3427046870806409921e-2_f64) * t102981 + t102984 - t102988 + F::cast_from(0.13170898365871023197e1_f64) * t28394 * t2772 - F::cast_from(0.14456046980341999104e-1_f64) * t95594 - F::cast_from(0.72280234901709995518e-2_f64) * t95598 + F::cast_from(0.96373646535613327357e-2_f64) * t102994 + F::cast_from(0.25702851531048074406e-1_f64) * t95604 - t95607 - F::cast_from(0.10975748638225852664e-1_f64) * t95613 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t28348 - F::cast_from(0.17135234354032049604e-2_f64) * t103001 + F::cast_from(0.14634331517634470219e-1_f64) * t95620 + t103007;
    (t103000, t103005, t103008)
}
