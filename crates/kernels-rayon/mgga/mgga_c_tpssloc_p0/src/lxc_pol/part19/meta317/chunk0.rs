//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1129/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1129(t3684: f64, t39500: f64, t2393: f64, t2528: f64, t677: f64, t9722: f64, t118: f64, t2375: f64, t3681: f64, t12110: f64, t9888: f64, t9467: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39502 = 0.86748650402413918736e-1_f64 * t3684 * t39500;
    let t39503 = t2393 * t2528;
    let t39505 = 0.12842595503380418954e1_f64 * t3684 * t39503;
    let t39506 = t677 * t9722;
    let t39508 = 0.38527786510141256862e1_f64 * t3684 * t39506;
    let t39510 = t3681 * t118 * t2375;
    let t39511 = 0.65061487801810439052e-1_f64 * t39510;
    let t39512 = t12110 * t9888;
    let t39513 = 0.19263893255070628431e1_f64 * t39512;
    let t39514 = t12110 * t9467;
    (t39502, t39503, t39505, t39506, t39508, t39511, t39513, t39514)
}
