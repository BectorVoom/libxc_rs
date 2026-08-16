//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1289/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1289(t2035: f64, t95019: f64, t28167: f64, t8996: f64, t9984: f64, t26090: f64, t7235: f64, t25082: f64, t49640: f64, t8717: f64, t25191: f64, t2322: f64, t25861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95020 = t95019 * t2035;
    let t95023 = 18.0_f64 * t28167 * t8996 * t9984;
    let t95025 = 3.0_f64 * t7235 * t26090;
    let t95032 = 9.0_f64 * t25082 * t8717 * t49640;
    let t95036 = 18.0_f64 * t7235 * t25191;
    let t95038 = 12.0_f64 * t2322 * t25861;
    (t95020, t95023, t95025, t95032, t95036, t95038)
}
