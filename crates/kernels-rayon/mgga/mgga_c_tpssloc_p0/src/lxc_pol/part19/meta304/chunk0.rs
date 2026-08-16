//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1091/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1091(t12512: f64, t580: f64, t1404: f64, t3931: f64, t1395: f64, t3946: f64, t12537: f64, t576: f64, t16: f64, t2: f64, t591: f64, t21: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39022 = t12512 * t580;
    let t39024 = t3931 * t1404;
    let t39026 = t1395 * t3946;
    let t39028 = t576 * t12537;
    let t39030 = 0.7464e2_f64 * t16;
    let t39031 = t2 * t591;
    let t39032 = 0.35904e3_f64 * t39031;
    let t39033 = t9 * t21;
    (t39022, t39024, t39026, t39028, t39030, t39031, t39032, t39033)
}
