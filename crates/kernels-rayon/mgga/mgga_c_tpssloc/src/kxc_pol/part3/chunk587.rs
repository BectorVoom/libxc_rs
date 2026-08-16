//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 587/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk587(t2765: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t291: f64, t888: f64, t892: f64, t914: f64, t287: f64, t891: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2784 = t2765 + 0.11872222222222222222e-1_f64 * t2766 - 0.11872222222222222222e-1_f64 * t2773 + 0.35616666666666666666e-1_f64 * t2778 - 0.17808333333333333333e-1_f64 * t2782;
    let t2786 = 0.621814e-1_f64 * t2784 * t291;
    let t2787 = t888 * t892;
    let t2789 = 2.0_f64 * t2787 * t914;
    let t2790 = t891 * t287;
    let t2791 = 1.0_f64 / t2790;
    let t2792 = t275 * t2791;
    (t2784, t2786, t2787, t2789, t2791, t2792)
}
