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
    let t25546 = F::new(0.66645927488835752265e2) * t8072 * t2786 - F::new(0.20734288552082234039e3) * t25504 * t917 - F::new(0.11721316454988582616e4) * t25508 + F::new(0.58606582274942913081e3) * t25511 + F::new(0.51573792401949763511e5) * t25515 * t2775 - F::new(0.25786896200974881756e5) * t25518 * t2781 - F::new(0.23181763972770020945e0) * t25522 + F::new(0.59710464543246456046e-2) * t25524 - F::new(0.12117441361606500412e2) * t2712 * t8063 - F::new(0.779739765264702906e1) * t25529 + F::new(0.8317224162823497664e2) * t25531 - F::new(0.57943328334337033725e4) * t25534 * t943 + F::new(0.15146801702008125515e1) * t25538 + F::new(0.15486228121497046737e3) * t3907 * t25540 * t25542;
    (t25540, t25546)
}
