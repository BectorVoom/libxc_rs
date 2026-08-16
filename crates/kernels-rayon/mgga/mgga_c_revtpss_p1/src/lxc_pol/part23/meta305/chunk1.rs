//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1567/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1567(t12295: f64, t12351: f64, t1159: f64, t3475: f64, t426: f64) -> (f64, f64, f64, f64) {
    let t12459 = 0.16068111111111111111e1_f64 * t12295;
    let t12460 = 0.46308888888888888888e0_f64 * t12351;
    let t12469 = 1.0_f64 / t3475 / t1159;
    let t12470 = t426 * t12469;
    (t12459, t12460, t12469, t12470)
}
