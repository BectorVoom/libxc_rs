//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2080/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2080(t10140: f64, t10143: f64, t2374: f64, t39354: f64, t39516: f64, t9879: f64, t9885: f64, t39325: f64, t39497: f64, t39500: f64, t39506: f64, t9882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40785 = t10140 * t10143;
    let t40790 = 0.21687162600603479684e-1_f64 * t2374 * t39354;
    let t40793 = 0.1301229756036208781e0_f64 * t2374 * t39516;
    let t40794 = t9879 * t9885;
    let t40797 = 0.38025319932552508021e2_f64 * t2374 * t39325;
    let t40799 = 0.67471172535210825684e-1_f64 * t2374 * t39497;
    let t40801 = 0.86748650402413918736e-1_f64 * t2374 * t39500;
    let t40803 = 0.38527786510141256862e1_f64 * t2374 * t39506;
    let t40804 = t9879 * t9882;
    (t40785, t40790, t40793, t40794, t40797, t40799, t40801, t40803, t40804)
}
