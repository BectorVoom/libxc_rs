//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 578/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk578(t10704: f64, t7064: f64, t3440: f64, t7137: f64, t3420: f64, t10669: f64, t10674: f64, t10679: f64, t10685: f64, t10688: f64, t10693: f64, t10696: f64, t10700: f64, t10703: f64, t1841: f64, t270: f64, t3434: f64, t681: f64, t9654: f64) -> (f64, f64) {
    let t10705 = t7064 * t10704;
    let t10706 = 0.32043859292259267849e-3_f64 * t10705;
    let t10708 = 0.30762104920568897135e-1_f64 * t7137 * t3440;
    let t10710 = 0.10254034973522965712e-1_f64 * t7137 * t3420;
    let t10711 = t9654 + 0.76905262301422242837e-2_f64 * t681 * t3434 - 0.76905262301422242837e-2_f64 * t270 * t10669 + 0.76905262301422242837e-2_f64 * t270 * t10674 + 0.85450291446024714263e-3_f64 * t1841 * t10679 + t10685 - 0.85450291446024714263e-3_f64 * t1841 * t10688 - t10693 + t10696 - t10700 + t10703 + t10706 - t10708 + t10710;
    (t10705, t10711)
}
