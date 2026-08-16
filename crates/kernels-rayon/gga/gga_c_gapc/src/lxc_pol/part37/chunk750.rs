//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 750/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk750(t1049: f64, t1617: f64, t3179: f64, t687: f64, t2011: f64, t1461: f64, t4043: f64, t1030: f64, t3141: f64, t5059: f64, t1044: f64, t1971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8610 = t1049 * t1617;
    let t8613 = t3179 * t687;
    let t8616 = t1049 * t2011;
    let t8619 = t1461 * t4043;
    let t8620 = t1030 * t8619;
    let t8621 = t3141 * t5059;
    let t8622 = t8620 * t8621;
    let t8624 = t1971 * t1044;
    (t8610, t8613, t8616, t8619, t8620, t8621, t8622, t8624)
}
