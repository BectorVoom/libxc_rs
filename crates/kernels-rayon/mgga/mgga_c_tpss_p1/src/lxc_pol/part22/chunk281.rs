//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 281/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk281(t833: f64, t859: f64, t839: f64, t850: f64, t855: f64, t863: f64) -> (f64, f64, f64) {
    let t898 = 0.301925e0_f64 * t833;
    let t901 = 0.82785e-1_f64 * t859;
    let t903 = 0.258925e1_f64 * t850 - t898 - 0.301925e0_f64 * t839 + 0.16504875e0_f64 * t855 - t901 - 0.82785e-1_f64 * t863;
    (t898, t901, t903)
}
