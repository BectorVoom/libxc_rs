//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 818/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk818(t3356: f64, t3413: f64, t1159: f64) -> (f64, f64, f64) {
    let t3459 = 0.68863333333333333333e0_f64 * t3356;
    let t3466 = 0.17365833333333333333e0_f64 * t3413;
    let t3475 = t1159 * t1159;
    (t3459, t3466, t3475)
}
