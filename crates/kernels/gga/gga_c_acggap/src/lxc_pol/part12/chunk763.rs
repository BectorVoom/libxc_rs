//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 763/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk763<F: Float>(t7462: F, t7515: F, t7519: F, t7539: F, t7464: F, t7466: F, t7468: F, t7473: F, t7479: F, t7481: F, t7484: F, t7488: F, t7491: F, t7496: F, t7500: F, t7504: F, t7524: F, t7529: F, t7531: F, t7536: F) -> (F, F, F, F, F) {
    let t8171 = F::cast_from(0.28582678745379824648e-3_f64) * t7462;
    let t8184 = F::cast_from(0.85748036236139473944e-3_f64) * t7515;
    let t8185 = F::cast_from(0.12579236915841660827e-2_f64) * t7519;
    let t8190 = F::cast_from(0.62896184579208304138e-3_f64) * t7539;
    let t8191 = -t8171 + F::cast_from(0.75475421495049964965e-2_f64) * t7464 - F::cast_from(0.11321313224257494745e-1_f64) * t7466 + F::cast_from(0.31448092289604152068e-2_f64) * t7468 + F::cast_from(0.15724046144802076034e-2_f64) * t7473 + F::cast_from(0.20965394859736101379e-2_f64) * t7479 - F::cast_from(0.12579236915841660828e-2_f64) * t7481 + F::cast_from(0.916875e-1_f64) * t7484 + F::cast_from(0.61125e-1_f64) * t7488 + F::cast_from(0.305625e-1_f64) * t7491 - F::cast_from(0.31448092289604152068e-2_f64) * t7496 + F::cast_from(0.12579236915841660828e-2_f64) * t7500 - F::cast_from(0.916875e-1_f64) * t7504 + t8184 - t8185 - F::cast_from(0.62896184579208304138e-3_f64) * t7524 - F::cast_from(0.83861579438944405517e-3_f64) * t7529 + F::cast_from(0.18868855373762491241e-2_f64) * t7531 + F::cast_from(0.94344276868812456207e-3_f64) * t7536 + t8190;
    (t8171, t8184, t8185, t8190, t8191)
}
