//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2279/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2279(t2121: f64, t3427: f64, t8010: f64, t24574: f64, t27416: f64, t27794: f64, t27441: f64, t85639: f64, t27446: f64, t1751: f64, t225: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94436 = t2121 * t3427 * t8010;
    let t94439 = 0.54831135561607547884e-2_f64 * t24574 * t27416;
    let t94446 = 0.54831135561607547884e-2_f64 * t24574 * t27794;
    let t94451 = 0.18277045187202515961e-2_f64 * t85639 * t27441;
    let t94456 = 0.36554090374405031922e-2_f64 * t85639 * t27446;
    let t94458 = t461 * t1751 * t225;
    (t94436, t94439, t94446, t94451, t94456, t94458)
}
