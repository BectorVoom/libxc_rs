//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 551/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk551(t2563: f64, t789: f64, t59: f64, t591: f64, t207: f64, t795: f64, t154: f64, t244: f64) -> (f64, f64, f64, f64) {
    let t2564 = t2563 * t789;
    let t2566 = t59 * t591;
    let t2569 = 0.26388888888888888888e-2_f64 * t2566 * t207 * t795;
    let t2570 = t154 * t244;
    (t2564, t2566, t2569, t2570)
}
