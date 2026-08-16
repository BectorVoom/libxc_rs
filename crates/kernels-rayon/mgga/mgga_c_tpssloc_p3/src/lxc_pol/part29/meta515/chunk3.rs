//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1888/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1888(t25373: f64, t25374: f64, t1530: f64, t606: f64, t25: f64, t4303: f64, t1408: f64, t776: f64, t868: f64, t1877: f64, t1915: f64, t2219: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25375 = t25373 * t25374;
    let t25377 = t606 * t1530;
    let t25381 = t25 * t4303;
    let t25385 = t1408 * t776;
    let t25392 = t1408 * t868;
    let t25397 = t1877 * t1915 * t2219;
    (t25375, t25377, t25381, t25385, t25392, t25397)
}
