//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1006/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1006(t15536: f64, t40826: f64, t72062: f64, t14451: f64, t1614: f64, t4669: f64, t72020: f64, t8636: f64, t72023: f64, t8902: f64, t8906: f64, t22: f64, t326: f64, t8041: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78188 = t40826 * t15536;
    let t78189 = 0.2993560425465952141e-1_f64 * t78188;
    let t78194 = 0.90915538847484472429e-2_f64 * t72062;
    let t78198 = t4669 * t14451 * t1614;
    let t78199 = 0.44903406381989282115e-1_f64 * t78198;
    let t78200 = t72020 * t8636;
    let t78201 = 0.27274661654245341728e-1_f64 * t78200;
    let t78202 = t72023 * t8902;
    let t78203 = 0.20455996240684006297e-1_f64 * t78202;
    let t78204 = t72020 * t8906;
    let t78205 = 0.27274661654245341729e-1_f64 * t78204;
    let t78207 = t326 * t8041 * t22;
    (t78189, t78194, t78199, t78201, t78203, t78205, t78207)
}
