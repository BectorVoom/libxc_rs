//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1470/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1470(t10647: f64, t291: f64, t2784: f64, t892: f64, t914: f64, t2787: f64, t2837: f64, t2841: f64, t888: f64) -> (f64, f64, f64, f64, f64) {
    let t10649 = 0.621814e-1_f64 * t10647 * t291;
    let t10650 = t2784 * t892;
    let t10652 = 3.0_f64 * t10650 * t914;
    let t10654 = 3.0_f64 * t2787 * t2837;
    let t10655 = t888 * t2841;
    (t10649, t10650, t10652, t10654, t10655)
}
