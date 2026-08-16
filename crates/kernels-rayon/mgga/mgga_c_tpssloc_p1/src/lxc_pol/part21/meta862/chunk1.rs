//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3130/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3130(t11588: f64, t6144: f64, t3447: f64, t3451: f64, t15402: f64, t18237: f64, t15376: f64, t15395: f64, t15406: f64, t3449: f64, t4900: f64, t4908: f64, t63294: f64, t63311: f64, t63372: f64, t63378: f64, t64756: f64, t64765: f64, t64770: f64, t64773: f64, t64775: f64) -> f64 {
    let t64779 = t11588 * t6144;
    let t64781 = t3447 * t64779 * t3451;
    let t64784 = t3447 * t15402 * t18237;
    let t64786 = -0.86419753086419753084e-3_f64 * t3447 * t15395 * t63372 - 0.1037037037037037037e-1_f64 * t3447 * t15395 * t63378 + 0.74074074074074074072e-3_f64 * t3447 * t4900 * t63311 + 0.55555555555555555554e-3_f64 * t3447 * t3449 * t64756 - 0.11111111111111111111e-2_f64 * t3447 * t4908 * t63294 + 0.18518518518518518518e-3_f64 * t64765 - 0.39506172839506172838e-2_f64 * t15376 * t15406 + 0.37037037037037037036e-3_f64 * t64770 + 0.37037037037037037036e-3_f64 * t64773 + 0.55555555555555555554e-3_f64 * t3447 * t64775 * t3451 + 0.18518518518518518518e-3_f64 * t64781 - 0.37037037037037037036e-3_f64 * t64784;
    t64786
}
