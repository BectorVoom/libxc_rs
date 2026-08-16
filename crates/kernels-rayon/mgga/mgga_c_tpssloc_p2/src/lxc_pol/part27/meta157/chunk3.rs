//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 863/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk863(t3242: f64, t461: f64, t2244: f64, t3440: f64, t337: f64, t51: f64, t1887: f64) -> (f64, f64, f64) {
    let t3441 = t461 * t3242;
    let t3442 = t3441 * t2244;
    let t3443 = t3440 * t3442;
    let t3446 = t51 * t337;
    let t3447 = t3446 * t1887;
    (t3442, t3443, t3447)
}
