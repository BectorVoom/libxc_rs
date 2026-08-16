//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1980/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1980<F: Float>(t30226: F, t689: F, t94768: F, t94763: F, t108279: F, t7515: F, t22453: F, t96463: F, t102372: F, t102378: F, t109573: F, t1903: F, t22415: F, t25921: F, t27837: F, t28815: F, t28888: F, t30267: F, t543: F, t7295: F, t7296: F, t7301: F, t7511: F, t96398: F, t96401: F, t96403: F) -> F {
    let t109630 = t30226 * t689;
    let t109631 = t94768 * t109630;
    let t109633 = t94763 * t109630;
    let t109647 = t108279 * t7515;
    let t109651 = t96463 * t22453;
    let t109656 = F::cast_from(0.14456046980341999104e-1_f64) * t109631 - F::cast_from(0.25702851531048074406e-1_f64) * t109633 + F::cast_from(0.4336814094102599731e0_f64) * t25921 * t30267 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t109573 * t543 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t28888 * t1903 - F::cast_from(0.52041769129231196772e1_f64) * t27837 * t28815 - F::cast_from(0.12851425765524037203e-1_f64) * t109647 + F::cast_from(0.13170898365871023197e1_f64) * t7511 * t22415 - t102372 + F::cast_from(0.19514881078765566037e-1_f64) * t109651 - F::cast_from(0.24093411633903331839e-3_f64) * t96398 - F::cast_from(0.34270468708064099208e-1_f64) * t102378 + t96401 + F::cast_from(0.11565819519348392139e-2_f64) * t96403;
    t109656
}
