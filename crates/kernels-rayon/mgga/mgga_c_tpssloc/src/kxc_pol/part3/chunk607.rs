//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 607/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk607(t340: f64, t697: f64, t344: f64, t221: f64, t339: f64, t135: f64, t976: f64) -> (f64, f64, f64, f64) {
    let t2965 = t697 * t340;
    let t2966 = t2965 * t344;
    let t2967 = t221 * t2966;
    let t2969 = 0.18518518518518518518e-3_f64 * t339 * t2967;
    let t2970 = t135 * t976;
    (t2965, t2967, t2969, t2970)
}
