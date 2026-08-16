//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2014/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2014(t23563: f64, t6740: f64, t10922: f64, t6717: f64, t3200: f64, t83015: f64, t1030: f64, t1058: f64, t3068: f64, t25511: f64, t6743: f64, t23592: f64, t23631: f64, t974: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83138 = t6740 * t23563;
    let t83157 = t6717 * t10922;
    let t83215 = t3200 * t83015;
    let t83220 = t1058 * sigma0 * t1030 * t3068;
    let t83233 = t6743 * t25511;
    let t83239 = t23631 * t974 * t23592;
    (t83138, t83157, t83215, t83220, t83233, t83239)
}
