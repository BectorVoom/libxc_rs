//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1098/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1098(t13602: f64, t12606: f64, t883: f64, t882: f64, t123: f64) -> (f64, f64, f64) {
    let t13603 = 2.0_f64 / 9.0_f64 * t13602;
    let t13611 = t883 * t12606;
    let t13612 = t882 * t13611;
    let t13613 = t123 * t13612;
    (t13603, t13611, t13613)
}
