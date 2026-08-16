//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1247/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1247(t1053: f64, t1102: f64, t1103: f64, t9005: f64, t37431: f64, t37438: f64, t40303: f64, t40305: f64, t40308: f64, t40313: f64, t40315: f64, t40320: f64, t40331: f64, t40334: f64, t42862: f64, t42866: f64, t42871: f64, t42874: f64) -> f64 {
    let t43854 = t1102 * t1053 * t1103 * t9005;
    let t43864 = 0.15243824895787514157e-3_f64 * t43854 - 0.38422568777328955684e-2_f64 * t40303 + 0.92232789896410962678e-3_f64 * t40305 + 0.72042316457491791906e-3_f64 * t40308 + t40313 - 0.86737941314158990623e-4_f64 * t40315 - t40320 + t42862 - 0.72042316457491791906e-3_f64 * t37431 + 0.10248087766267884742e-3_f64 * t37438 + t42866 + 0.16260079888840015101e-2_f64 * t40331 - 0.3903207359137154578e-3_f64 * t40334 - t42871 + t42874;
    t43864
}
