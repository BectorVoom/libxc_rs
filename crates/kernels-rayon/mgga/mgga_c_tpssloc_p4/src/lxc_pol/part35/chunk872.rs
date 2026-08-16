//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 872/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk872(t11778: f64, t61: f64, t121: f64, t3584: f64, t1229: f64, t676: f64, t486: f64, t11552: f64, t221: f64, t456: f64, t1176: f64, t3242: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11779 = t61 * t11778;
    let t11784 = t121 * t3584;
    let t11789 = t676 * t1229;
    let t11818 = t676 * t486;
    let t11832 = t221 * t11552;
    let t11834 = 5.0_f64 / 1296.0_f64 * t456 * t11832;
    let t11848 = t1176 * t3242;
    (t11779, t11784, t11789, t11818, t11832, t11834, t11848)
}
