//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1237/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1237(t7898: f64, t871: f64, t938: f64, t2367: f64, t8062: f64, t913: f64, t2670: f64, t7481: f64, t1: f64, t1885: f64, t24468: f64, t25504: f64, t25508: f64, t25511: f64, t25515: f64, t25518: f64, t25522: f64, t25524: f64, t25529: f64, t25531: f64, t2712: f64, t2775: f64, t2781: f64, t2786: f64, t3907: f64, t8063: f64, t8072: f64, t917: f64, t943: f64) -> (f64, f64) {
    let t25534 = t938 * t7898 * t871;
    let t25538 = t913 * t2367 * t8062;
    let t25540 = t7481 * t2670;
    let t25542 = t24468 * t1885 * t1;
    let t25546 = 0.66645927488835752265e2_f64 * t8072 * t2786 - 0.20734288552082234039e3_f64 * t25504 * t917 - 0.11721316454988582616e4_f64 * t25508 + 0.58606582274942913081e3_f64 * t25511 + 0.51573792401949763511e5_f64 * t25515 * t2775 - 0.25786896200974881756e5_f64 * t25518 * t2781 - 0.23181763972770020945e0_f64 * t25522 + 0.59710464543246456046e-2_f64 * t25524 - 0.12117441361606500412e2_f64 * t2712 * t8063 - 0.779739765264702906e1_f64 * t25529 + 0.8317224162823497664e2_f64 * t25531 - 0.57943328334337033725e4_f64 * t25534 * t943 + 0.15146801702008125515e1_f64 * t25538 + 0.15486228121497046737e3_f64 * t3907 * t25540 * t25542;
    (t25540, t25546)
}
