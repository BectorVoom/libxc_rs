//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1269/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1269(t11104: f64, t2156: f64, t1147: f64, t1306: f64, t26780: f64, t2993: f64, t30767: f64, t30769: f64, t30772: f64, t30775: f64, t30778: f64, t30780: f64, t30784: f64, t30786: f64, t803: f64, t9725: f64) -> f64 {
    let t31014 = t11104 * t2156;
    let t31017 = -3.0_f64 * t1147 * t1306 * t26780 - 3.0_f64 * t1306 * t2993 * t9725 - t1306 * t31014 * t803 - t30767 + t30769 - t30772 - t30775 + t30778 + t30780 - t30784 - t30786;
    t31017
}
