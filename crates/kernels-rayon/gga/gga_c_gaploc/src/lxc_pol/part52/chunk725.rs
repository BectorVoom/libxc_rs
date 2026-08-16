//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 725/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk725(t14384: f64, t2580: f64, t14377: f64, t738: f64, t13488: f64, t13490: f64, t13494: f64, t13497: f64, t13501: f64, t13504: f64, t13509: f64, t13935: f64, t13938: f64, t2508: f64, t270: f64) -> (f64, f64, f64) {
    let t14415 = t2580 * t14384;
    let t14420 = t738 * t14377;
    let t14425 = 0.30762104920568897134e-1_f64 * t2508 * t14415 - 0.1281754371690370714e-2_f64 * t13935 - t13488 - 0.96131577876777803547e-3_f64 * t13490 + t13494 + t13497 + t13501 - 0.76905262301422242837e-2_f64 * t270 * t14420 + 0.64087718584518535698e-3_f64 * t13504 - t13509 + 0.1281754371690370714e-2_f64 * t13938;
    (t14415, t14420, t14425)
}
