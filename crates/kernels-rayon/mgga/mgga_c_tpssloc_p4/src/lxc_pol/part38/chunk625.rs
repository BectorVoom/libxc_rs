//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 625/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk625(t2770: f64, t344: f64, t2244: f64, t2979: f64, t337: f64, t39: f64, t1887: f64) -> (f64, f64, f64) {
    let t2980 = t344 * t2770;
    let t2981 = t2980 * t2244;
    let t2982 = t2979 * t2981;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    (t2981, t2982, t2986)
}
