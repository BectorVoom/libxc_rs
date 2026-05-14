//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 802/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk802<F: Float>(t213: F, t2760: F, t215: F, t231: F, t268: F, t836: F, t2798: F, t2722: F, t675: F, t251: F, t4503: F, t786: F, t2723: F, t2453: F, t2797: F, t281: F, t68: F) -> (F, F, F, F, F, F) {
    let t10513 = t213 * t2760;
    let t10518 = t268 * t215 * t836 * t231;
    let t10519 = t2798 * t10518;
    let t10521 = t675 * t2722;
    let t10523 = t268 * t10521 * t231;
    let t10524 = t2798 * t10523;
    let t10529 = t4503 * t251;
    let t10530 = t786 * t10529;
    let t10532 = t268 * t10521 * t2723;
    let t10533 = t10530 * t10532;
    let t10535 = t2453 * t2797;
    let t10538 = t281 * t68 * t836 * t231;
    (t10513, t10519, t10524, t10533, t10535, t10538)
}
