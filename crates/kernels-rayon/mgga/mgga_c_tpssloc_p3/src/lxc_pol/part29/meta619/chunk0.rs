//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2061/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2061(t24574: f64, t24765: f64, t27561: f64, t7327: f64, t24826: f64, t24834: f64, t210: f64, t24810: f64, t24848: f64, t24807: f64, t225: f64, t24705: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86001 = t24574 * t24765;
    let t86015 = t7327 * t27561;
    let t86020 = t24826 * t24834;
    let t86036 = t24810 * t210;
    let t86037 = t86036 * t24848;
    let t86057 = t24826 * t24807;
    let t86059 = t24705 * t225;
    (t86001, t86015, t86020, t86036, t86037, t86057, t86059)
}
