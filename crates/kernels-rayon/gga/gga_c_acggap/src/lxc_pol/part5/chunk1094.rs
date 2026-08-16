//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1094/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1094(t5351: f64, t5371: f64, t1220: f64, t1914: f64, t863: f64, t864: f64, t316: f64, t322: f64, t449: f64, t6557: f64, t1907: f64, t862: f64, t865: f64) -> (f64, f64, f64, f64) {
    let t19607 = t5371 * t5351;
    let t19611 = t863 * t1220 * t1914 * t864;
    let t19615 = t316 * t449 * t6557 * t322;
    let t19618 = t862 * t1907 * t865;
    (t19607, t19611, t19615, t19618)
}
