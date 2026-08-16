//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1102/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1102(t10928: f64, t6574: f64, t822: f64, t123: f64, t15499: f64, t21503: f64, t883: f64, t2194: f64, t9981: f64, t2012: f64, t7809: f64, t9801: f64) -> (f64, f64, f64, f64, f64) {
    let t28640 = t822 * t10928 * t6574;
    let t28641 = t15499 * t123;
    let t28645 = 0.46011511144704899612e1_f64 * t28640 * t28641 * t883 * t21503;
    let t28659 = t2194 * t9981;
    let t28673 = t2012 * t7809;
    let t28675 = 0.38342925953920749676e1_f64 * t28673 * t9801;
    (t28640, t28645, t28659, t28673, t28675)
}
