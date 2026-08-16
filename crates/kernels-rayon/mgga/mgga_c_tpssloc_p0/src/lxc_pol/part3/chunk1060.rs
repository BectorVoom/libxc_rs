//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1060/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1060(t13563: f64, t10296: f64, t10298: f64, t10302: f64, t13566: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64) -> (f64, f64) {
    let t13679 = 0.13418888888888888889e0_f64 * t13563;
    let t13692 = -0.40256666666666666667e0_f64 * t13566 - 0.33547222222222222222e0_f64 * t13569 + 0.12077e1_f64 * t13572 - 0.40256666666666666666e0_f64 * t13575 - 0.20128333333333333333e0_f64 * t13578 - 0.181155e1_f64 * t13581 + 0.12077e1_f64 * t13584 + 0.60385e0_f64 * t13587 - 0.18396666666666666667e0_f64 * t10296 + 0.5519e-1_f64 * t10302 + 0.18396666666666666667e-1_f64 * t10298;
    (t13679, t13692)
}
