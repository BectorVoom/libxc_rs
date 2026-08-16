//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2407/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2407(t2531: f64, t9919: f64, t9467: f64, t9879: f64, t2374: f64, t39519: f64, t39503: f64, t118: f64, t2375: f64, t2448: f64, t39391: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40733 = t2531 * t9919;
    let t40738 = t9879 * t9467;
    let t40741 = 0.43374325201206959368e-1_f64 * t2374 * t39519;
    let t40743 = 0.12842595503380418954e1_f64 * t2374 * t39503;
    let t40745 = t2448 * t118 * t2375;
    let t40748 = 0.35089341735807877242e1_f64 * t761 * t39391;
    (t40733, t40738, t40741, t40743, t40745, t40748)
}
