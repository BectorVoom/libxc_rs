//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 620/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk620(t321: f64, t3601: f64, t123: f64, t6118: f64, t11627: f64, t550: f64, t5539: f64, t11622: f64, t1843: f64, t10734: f64, t10740: f64, t10744: f64, t10746: f64, t10750: f64, t1841: f64, t3604: f64, t3617: f64, t650: f64, t681: f64) -> (f64, f64, f64, f64, f64) {
    let t11679 = t321 * t3601;
    let t11680 = t11679 * t123;
    let t11681 = t11680 * t6118;
    let t11684 = t550 * t11627;
    let t11685 = t5539 * t11684;
    let t11688 = t550 * t11622;
    let t11689 = t1843 * t11688;
    let t11697 = 0.10254034973522965712e-1_f64 * t650 * t3604 + 0.10254034973522965712e-1_f64 * t650 * t3617 + 0.76905262301422242837e-2_f64 * t681 * t3604 + 0.25635087433807414279e-2_f64 * t1841 * t11681 - 0.17090058289204942852e-2_f64 * t1841 * t11685 + 0.85450291446024714263e-3_f64 * t1841 * t11689 + 0.17090058289204942853e-2_f64 * t10734 - 0.1281754371690370714e-2_f64 * t10740 - 0.1281754371690370714e-2_f64 * t10744 + 0.1281754371690370714e-2_f64 * t10746 + 0.1281754371690370714e-2_f64 * t10750;
    (t11679, t11680, t11684, t11688, t11697)
}
