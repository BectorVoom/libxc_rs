//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1098/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1098(t4872: f64, t45304: f64, t58969: f64, t73956: f64, t73958: f64, t85518: f64, t85526: f64, t85533: f64, t85540: f64, t85548: f64, t85556: f64, t85560: f64, t86744: f64, t86747: f64, t86750: f64) -> (f64, f64) {
    let t87975 = t4872 * t4872;
    let t87994 = -0.11554466666666666666e1_f64 * t73956 + 0.38514888888888888888e0_f64 * t73958 - 0.38514888888888888888e0_f64 * t58969 + 0.234754e0_f64 * t86750 - 0.352131e0_f64 * t86747 - 0.42794320987654320987e0_f64 * t85556 - 0.14443083333333333333e0_f64 * t85560 - 0.44016375e0_f64 * t86744 + 0.19257444444444444444e1_f64 * t85518 - 0.34663399999999999999e1_f64 * t85526 - 0.28886166666666666666e0_f64 * t85533 + 0.34663399999999999999e1_f64 * t85540 + 0.86658499999999999998e0_f64 * t85548 + 0.59912049382716049381e0_f64 * t45304;
    (t87975, t87994)
}
