//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 896/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk896<F: Float>(t31100: F, t33953: F, t5127: F, t13287: F, t31057: F, t4210: F, t13364: F, t13299: F, t31115: F, t33938: F, t7433: F, t8779: F, t1181: F, t21955: F, t30806: F, t599: F) -> (F, F, F, F, F, F, F, F) {
    let t35279 = 0.42874018118069736972e-2 * t31100;
    let t35284 = t33953 * t5127;
    let t35286 = t31057 * t13287 * t35284;
    let t35288 = t33953 * t4210;
    let t35290 = t31057 * t13364 * t35288;
    let t35301 = t31115 * t13299 * t33938;
    let t35307 = t7433 * t8779;
    let t35315 = t30806 * t1181 * t599 * t21955;
    (t35279, t35284, t35286, t35288, t35290, t35301, t35307, t35315)
}
