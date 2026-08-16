//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1111/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1111(t40: f64, t4100: f64, t4102: f64, t185: f64, t5392: f64, t2658: f64, t1484: f64, t4310: f64, t1462: f64, t4205: f64, t2433: f64, t5398: f64, t73: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t5497 = 2.0_f64 * t4100;
    let t5498 = 8.0_f64 * t4102;
    let t5499 = t185 * t5392;
    let t5501 = 12.0_f64 * t2658 * t5499;
    let t5502 = t4310 * t1484;
    let t5506 = 8.0_f64 * t4205 * t1462;
    let t5512 = piecewise3(t146, 0.0_f64, 4.0_f64 / 9.0_f64 * t2433 * t5392 + 4.0_f64 / 3.0_f64 * t73 * t5398);
    (t5497, t5498, t5499, t5501, t5502, t5506, t5512)
}
