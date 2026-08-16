//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 353/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk353(t1132: f64, t1134: f64, t1118: f64, t406: f64) -> (f64, f64, f64) {
    let t1135 = t1132 * t1134;
    let t1137 = 0.29896666666666666667e0_f64 * t1118;
    let t1139 = f64::sqrt(t406);
    (t1135, t1137, t1139)
}
