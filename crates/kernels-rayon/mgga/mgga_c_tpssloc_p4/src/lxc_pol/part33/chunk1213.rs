//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1213/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1213(t1834: f64, t6387: f64, t20553: f64, t562: f64, t20489: f64, t1824: f64, t6434: f64, t20193: f64, t604: f64, t1453: f64, t5488: f64, t112: f64, t22430: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t74941 = t1834 * t6387;
    let t74949 = t562 * t20553;
    let t74967 = t562 * t20489;
    let t75026 = t6434 * t1824;
    let t75284 = t20193 * t604;
    let t75603 = t1453 * t5488;
    let t75784 = t22430 * t112;
    (t74941, t74949, t74967, t75026, t75284, t75603, t75784)
}
