//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1096/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1096(t10296: f64, t10298: f64, t10302: f64, t13567: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t1540: f64, t2394: f64) -> (f64, f64) {
    let t13592 = -t13567 - 0.33218518518518518518e0_f64 * t13569 + 0.11958666666666666667e1_f64 * t13572 - 0.39862222222222222222e0_f64 * t13575 - 0.19931111111111111111e0_f64 * t13578 - 0.17938e1_f64 * t13581 + 0.11958666666666666667e1_f64 * t13584 + 0.59793333333333333334e0_f64 * t13587 - 0.18257037037037037037e0_f64 * t10296 + 0.54771111111111111111e-1_f64 * t10302 + 0.18257037037037037037e-1_f64 * t10298;
    let t13598 = t2394 * t1540;
    (t13592, t13598)
}
