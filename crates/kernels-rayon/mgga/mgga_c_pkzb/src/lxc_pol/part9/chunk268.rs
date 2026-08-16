//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 268/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk268(t833: f64, t853: f64, t819: f64, t826: f64) -> (f64, f64, f64) {
    let t855 = 1.0_f64 * t833 * t853;
    let t856 = 0.17123333333333333333e-1_f64 * t819;
    let t858 = -t856 + 0.5137e-1_f64 * t826;
    (t855, t856, t858)
}
