//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 768/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk768(t8562: f64, t8564: f64, t8758: f64, t8917: f64, t118: f64, t2127: f64, t2163: f64, t508: f64, t569: f64, t8456: f64, t8463: f64, t8597: f64, t8601: f64, t8743: f64, t8750: f64, t8765: f64, t8964: f64) -> (f64, f64) {
    let t8967 = t8917 + 4.0_f64 * t8758 + t8562 + t8564;
    let t8970 = -t118 * t8964 - 2.0_f64 * t2127 * t2163 - t508 * t8917 + t569 * t8967 - t8456 - t8463 + t8597 - t8601 - 4.0_f64 * t8743 - 4.0_f64 * t8750 + 2.0_f64 * t8765;
    (t8967, t8970)
}
