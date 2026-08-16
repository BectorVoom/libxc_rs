//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 927/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk927(t3375: f64, t3665: f64, t3775: f64, t3806: f64, t1029: f64, t3237: f64, t1020: f64, t3228: f64, t879: f64, t1036: f64, t174: f64, t386: f64, t387: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14239 = t3375 * t3665;
    let t14242 = 0.51448821741683684368e-2_f64 * t3775 * t3806;
    let t14243 = t3237 * t1029;
    let t14245 = t3228 * t1020;
    let t14255 = t879 * t879;
    let t14260 = 0.12862205435420921092e-2_f64 * t1036 * t386 * t387 * t174 * t14255;
    (t14239, t14242, t14243, t14245, t14255, t14260)
}
