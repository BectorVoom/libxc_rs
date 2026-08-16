//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1414/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1414(t174: f64, t740: f64, t9323: f64, t447: f64, t637: f64, t446: f64, t1295: f64, t4534: f64, t233: f64, t1655: f64, t2791: f64, t5399: f64, t911: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t18374 = 2.0_f64 * t740;
    let t18375 = 6.0_f64 * t9323;
    let t18376 = -t18374 + t18375;
    let t18377 = piecewise3(t175, 0.0_f64, t18376);
    let t18378 = t447 * t18377;
    let t18379 = t18378 * t637;
    let t18380 = t446 * t18379;
    let t18382 = t4534 * t1295;
    let t18383 = t233 * t18382;
    let t18385 = t1655 * t2791;
    let t18386 = t911 * t5399;
    (t18376, t18380, t18383, t18385, t18386)
}
