//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2035/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2035(t86928: f64, t6562: f64, t7488: f64, t82133: f64, t25225: f64, t6547: f64, t23168: f64, t25338: f64, t23012: f64, t7485: f64, t25046: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86929 = 0.82246703342411321824e-2_f64 * t86928;
    let t86940 = t6562 * t82133 * t7488;
    let t86941 = 0.82246703342411321824e-2_f64 * t86940;
    let t86942 = t6547 * t25225;
    let t86943 = 0.38381794893125283518e-1_f64 * t86942;
    let t86950 = t23168 * t25338;
    let t86951 = 0.76763589786250567036e-1_f64 * t86950;
    let t86955 = t23012 * t7485;
    let t86967 = t6579 * t25046;
    (t86929, t86941, t86943, t86951, t86955, t86967)
}
