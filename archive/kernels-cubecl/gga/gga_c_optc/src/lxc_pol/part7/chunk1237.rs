//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1237/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1237<F: Float>(t7898: F, t871: F, t938: F, t2367: F, t8062: F, t913: F, t2670: F, t7481: F, t1: F, t1885: F, t24468: F, t25504: F, t25508: F, t25511: F, t25515: F, t25518: F, t25522: F, t25524: F, t25529: F, t25531: F, t2712: F, t2775: F, t2781: F, t2786: F, t3907: F, t8063: F, t8072: F, t917: F, t943: F) -> (F, F) {
    let t25534 = t938 * t7898 * t871;
    let t25538 = t913 * t2367 * t8062;
    let t25540 = t7481 * t2670;
    let t25542 = t24468 * t1885 * t1;
    let t25546 = F::cast_from(0.66645927488835752265e2_f64) * t8072 * t2786 - F::cast_from(0.20734288552082234039e3_f64) * t25504 * t917 - F::cast_from(0.11721316454988582616e4_f64) * t25508 + F::cast_from(0.58606582274942913081e3_f64) * t25511 + F::cast_from(0.51573792401949763511e5_f64) * t25515 * t2775 - F::cast_from(0.25786896200974881756e5_f64) * t25518 * t2781 - F::cast_from(0.23181763972770020945e0_f64) * t25522 + F::cast_from(0.59710464543246456046e-2_f64) * t25524 - F::cast_from(0.12117441361606500412e2_f64) * t2712 * t8063 - F::cast_from(0.779739765264702906e1_f64) * t25529 + F::cast_from(0.8317224162823497664e2_f64) * t25531 - F::cast_from(0.57943328334337033725e4_f64) * t25534 * t943 + F::cast_from(0.15146801702008125515e1_f64) * t25538 + F::cast_from(0.15486228121497046737e3_f64) * t3907 * t25540 * t25542;
    (t25540, t25546)
}
