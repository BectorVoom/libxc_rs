//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2157/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2157<F: Float>(t19912: F, t7111: F, t100121: F, t100146: F, t19626: F, t19641: F, t19731: F, t19819: F, t19873: F, t19944: F, t25517: F, t25539: F, t25569: F, t27498: F, t27536: F, t4788: F, t6268: F, t6293: F, t6327: F, t93548: F, t93821: F) -> F {
    let t107169 = t7111 * t19912;
    let t107183 = F::cast_from(0.47637797908966374413e-3_f64) * t25569 * t6327 - F::cast_from(0.25724410870841842183e-2_f64) * t100121 * t19819 + F::cast_from(0.17149607247227894789e-2_f64) * t27536 * t19944 - t25539 * t6293 / F::new(81.0) + t107169 / F::new(648.0) + F::cast_from(0.85748036236139473944e-3_f64) * t93548 * t19641 - F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t19873 + F::cast_from(0.57165357490759649296e-3_f64) * t100146 * t4788 + F::cast_from(0.57165357490759649296e-3_f64) * t93821 * t6268 + F::cast_from(0.57165357490759649296e-3_f64) * t25517 * t19731 - F::cast_from(0.28582678745379824648e-3_f64) * t27498 * t19626;
    t107183
}
