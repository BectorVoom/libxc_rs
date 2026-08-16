//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 744/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk744(t5607: f64, t7195: f64, t5587: f64, t5608: f64, t7178: f64, t12: f64, t397: f64, t52: f64, t428: f64, t7318: f64, t11: f64, t1690: f64, t53: f64, t5555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32179 = t7195 * t5607;
    let t32181 = 0.11352761063935582948e-3_f64 * t5587 * t32179;
    let t32185 = 0.25537443351851851852e-1_f64 * t7178 * t5608;
    let t32186 = t12 * t397;
    let t32187 = t52 * t32186;
    let t32190 = t7318 * t428;
    let t32208 = t1690 * t11 * t5555 * t53;
    (t32179, t32181, t32185, t32186, t32187, t32190, t32208)
}
