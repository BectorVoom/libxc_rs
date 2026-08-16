//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1878/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1878(t13141: f64, t460: f64, t12051: f64, t3603: f64, t13128: f64, t11239: f64, t13038: f64) -> (f64, f64, f64, f64) {
    let t13142 = t460 * t13141;
    let t13143 = t12051 * t3603;
    let t13144 = t13128 * t13143;
    let t13147 = t11239 * t13038;
    (t13142, t13143, t13144, t13147)
}
