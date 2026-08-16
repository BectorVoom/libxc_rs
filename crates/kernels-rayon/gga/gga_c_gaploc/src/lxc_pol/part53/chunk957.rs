//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 957/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk957(t12270: f64, t1960: f64, t977: f64, t2595: f64, t38892: f64, t12272: f64, t7324: f64, t3749: f64, t7822: f64, t38885: f64, t2728: f64, t2358: f64, t39337: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47083 = t1960 * t12270 * t977;
    let t47085 = t38892 * t2595;
    let t47087 = t7324 * t12272;
    let t47096 = t7822 * t3749;
    let t47097 = t38885 * t977;
    let t47105 = t1960 * t3749 * t2728;
    let t47107 = t39337 * t2358;
    (t47083, t47085, t47087, t47096, t47097, t47105, t47107)
}
