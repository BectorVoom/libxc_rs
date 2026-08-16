//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 570/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk570(t1176: f64, t461: f64, t491: f64, t225: f64, t497: f64, t457: f64, t1240: f64, t1193: f64, t2127: f64, t210: f64, t2120: f64, t2132: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7284 = t1176 * t461;
    let t7285 = t7284 * t491;
    let t7286 = t225 * t497;
    let t7299 = t457 * t461;
    let t7300 = t7299 * t491;
    let t7301 = t225 * t1240;
    let t7309 = t2127 * t1193 / 288.0_f64;
    let t7310 = t2120 * t210;
    let t7313 = t2132 * t52;
    (t7284, t7285, t7286, t7299, t7300, t7301, t7309, t7310, t7313)
}
