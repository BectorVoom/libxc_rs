//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1197/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1197(t40736: f64, t9467: f64, t9879: f64, t2374: f64, t39519: f64, t39503: f64, t118: f64, t2375: f64, t2448: f64, t39391: f64, t761: f64, t2427: f64, t9926: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40737 = 16.0_f64 * t40736;
    let t40738 = t9879 * t9467;
    let t40739 = 0.86748650402413918736e-1_f64 * t40738;
    let t40741 = 0.43374325201206959368e-1_f64 * t2374 * t39519;
    let t40743 = 0.12842595503380418954e1_f64 * t2374 * t39503;
    let t40745 = t2448 * t118 * t2375;
    let t40746 = 0.65061487801810439052e-1_f64 * t40745;
    let t40748 = 0.35089341735807877242e1_f64 * t761 * t39391;
    let t40750 = 16.0_f64 * t2427 * t9926;
    (t40737, t40739, t40741, t40743, t40746, t40748, t40750)
}
