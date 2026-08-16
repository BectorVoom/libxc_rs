//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 557/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk557(t14494: f64, t338: f64, t118: f64, t2123: f64, t2211: f64, t3204: f64, t321: f64, t2085: f64, t3191: f64, t2228: f64, t326: f64, t650: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14495 = t338 * t14494;
    let t14496 = t118 * t14495;
    let t14498 = t2211 * t2123;
    let t14500 = 0.39914139006212695214e-1_f64 * t118 * t14498;
    let t14501 = t3204 * t321;
    let t14504 = t3191 * t2085;
    let t14505 = 0.90915538847484472429e-2_f64 * t14504;
    let t14506 = t326 * t2228;
    let t14507 = t14506 * t650;
    (t14495, t14496, t14498, t14500, t14501, t14505, t14506, t14507)
}
