//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1980/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1980(t30226: f64, t689: f64, t94768: f64, t94763: f64, t108279: f64, t7515: f64, t22453: f64, t96463: f64, t102372: f64, t102378: f64, t109573: f64, t1903: f64, t22415: f64, t25921: f64, t27837: f64, t28815: f64, t28888: f64, t30267: f64, t543: f64, t7295: f64, t7296: f64, t7301: f64, t7511: f64, t96398: f64, t96401: f64, t96403: f64) -> f64 {
    let t109630 = t30226 * t689;
    let t109631 = t94768 * t109630;
    let t109633 = t94763 * t109630;
    let t109647 = t108279 * t7515;
    let t109651 = t96463 * t22453;
    let t109656 = 0.14456046980341999104e-1_f64 * t109631 - 0.25702851531048074406e-1_f64 * t109633 + 0.4336814094102599731e0_f64 * t25921 * t30267 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t109573 * t543 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t28888 * t1903 - 0.52041769129231196772e1_f64 * t27837 * t28815 - 0.12851425765524037203e-1_f64 * t109647 + 0.13170898365871023197e1_f64 * t7511 * t22415 - t102372 + 0.19514881078765566037e-1_f64 * t109651 - 0.24093411633903331839e-3_f64 * t96398 - 0.34270468708064099208e-1_f64 * t102378 + t96401 + 0.11565819519348392139e-2_f64 * t96403;
    t109656
}
