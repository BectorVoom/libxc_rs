//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1169/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1169(t14704: f64, t1089: f64, t12606: f64, t1088: f64, t123: f64) -> (f64, f64, f64) {
    let t14705 = 0.20128333333333333334e0_f64 * t14704;
    let t14706 = t1089 * t12606;
    let t14707 = t1088 * t14706;
    let t14708 = t123 * t14707;
    (t14705, t14706, t14708)
}
