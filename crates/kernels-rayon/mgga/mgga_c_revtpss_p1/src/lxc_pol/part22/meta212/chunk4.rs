//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1345/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1345(t1168: f64, t5146: f64, t3358: f64, t3483: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64) -> (f64, f64) {
    let t5147 = t5146 * t1168;
    let t5155 = t3483 - 0.30902777777777777778e-2_f64 * t3358 - 0.30902777777777777778e-2_f64 * t5044 - 0.61805555555555555555e-2_f64 * t5049 + 0.18541666666666666667e-1_f64 * t5054 + 0.92708333333333333333e-2_f64 * t5058;
    (t5147, t5155)
}
