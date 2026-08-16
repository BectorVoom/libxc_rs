//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1306/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1306(t5705: f64, t7309: f64, t19809: f64, t60960: f64, t1288: f64, t2428: f64, t2: f64, t823: f64, t555: f64, t750: f64, t3724: f64, t580: f64) -> (f64, f64, f64, f64, f64) {
    let t63710 = t5705 * t7309;
    let t63766 = t60960 * t19809;
    let t63771 = t1288 * t2428;
    let t63783 = t823 * t2;
    let t63785 = t63783 * t555 * t750;
    let t63791 = t580 * t3724;
    (t63710, t63766, t63771, t63785, t63791)
}
