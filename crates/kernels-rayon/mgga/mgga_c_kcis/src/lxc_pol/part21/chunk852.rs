//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 852/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk852(t13150: f64, t4555: f64, t3210: f64, t4554: f64, t1774: f64, t9568: f64, t1092: f64, t3178: f64, t5019: f64, t3198: f64, t4992: f64, t86: f64) -> (f64, f64, f64, f64, f64) {
    let t13161 = t4555 * t13150;
    let t13162 = t3210 * t13161;
    let t13163 = t4554 * t13162;
    let t13165 = t9568 * t1774;
    let t13166 = t1092 * t13165;
    let t13168 = t3178 * t5019;
    let t13169 = t1092 * t13168;
    let t13172 = t86 * t4992 * t3198;
    (t13161, t13163, t13166, t13169, t13172)
}
