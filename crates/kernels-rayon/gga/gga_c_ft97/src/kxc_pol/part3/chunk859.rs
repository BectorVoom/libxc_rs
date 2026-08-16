//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 859/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk859(t17220: f64, t17225: f64, t17246: f64, t17353: f64, t605: f64, t144: f64, t1882: f64, t4819: f64, t4815: f64, t3478: f64, t925: f64, t9144: f64) -> (f64, f64, f64, f64, f64) {
    let t17355 = t17220 + t17225 + t17246 + t17353;
    let t17356 = t605 * t17355;
    let t17357 = t144 * t17356;
    let t17360 = t1882 * t4819;
    let t17362 = t1882 * t4815;
    let t17365 = t925 * t3478;
    let t17366 = t9144 * t17365;
    (t17356, t17357, t17360, t17362, t17366)
}
