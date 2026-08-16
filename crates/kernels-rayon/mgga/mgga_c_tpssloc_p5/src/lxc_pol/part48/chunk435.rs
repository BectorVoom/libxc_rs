//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 435/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk435(t3071: f64, t3072: f64, t225: f64, t3020: f64, t68: f64, t369: f64, t374: f64, t376: f64, t677: f64, t370: f64, t35: f64, t365: f64, t612: f64) -> (f64, f64, f64, f64, f64) {
    let t3073 = t3071 * t3072;
    let t3076 = t3020 * t225;
    let t3077 = t3076 * t68;
    let t3078 = t3077 * t369;
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0_f64;
    let t3087 = 1.0_f64 / t35 / t365 / t612;
    (t3073, t3076, t3078, t3084, t3087)
}
