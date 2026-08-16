//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 502/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk502(t14077: f64, t262: f64, t2134: f64, t78: f64, t8: f64, t271: f64, t4765: f64) -> (f64, f64, f64, f64, f64) {
    let t14078 = t14077 * t262;
    let t14079 = t2134 * t14078;
    let t14081 = t78 * t8;
    let t14082 = 1.0_f64 / t14081;
    let t14083 = t14082 * t271;
    let t14084 = t4765 * t14083;
    (t14078, t14079, t14082, t14083, t14084)
}
