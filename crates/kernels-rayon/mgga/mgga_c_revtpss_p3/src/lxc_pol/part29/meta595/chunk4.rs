//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1999/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1999(t102993: f64, t25431: f64, t2439: f64, t8011: f64, t93170: f64, t28347: f64, t686: f64, t72: f64, t25387: f64, t102981: f64, t102984: f64, t102988: f64, t25383: f64, t2772: f64, t28348: f64, t28394: f64, t95594: f64, t95598: f64, t95604: f64, t95607: f64, t95613: f64, t95620: f64) -> (f64, f64, f64) {
    let t102994 = t25431 * t102993;
    let t103000 = t8011 * t2439;
    let t103001 = t93170 * t103000;
    let t103005 = t28347 * t72 * t686;
    let t103007 = 0.51405703062096148812e-1_f64 * t25387 * t103005;
    let t103008 = -0.3427046870806409921e-2_f64 * t102981 + t102984 - t102988 + 0.13170898365871023197e1_f64 * t28394 * t2772 - 0.14456046980341999104e-1_f64 * t95594 - 0.72280234901709995518e-2_f64 * t95598 + 0.96373646535613327357e-2_f64 * t102994 + 0.25702851531048074406e-1_f64 * t95604 - t95607 - 0.10975748638225852664e-1_f64 * t95613 + 0.17347256376410398924e1_f64 * t25383 * t28348 - 0.17135234354032049604e-2_f64 * t103001 + 0.14634331517634470219e-1_f64 * t95620 + t103007;
    (t103000, t103005, t103008)
}
