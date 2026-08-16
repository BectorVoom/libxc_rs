//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1019/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1019(t76050: f64, t76054: f64, t70358: f64, t76066: f64, t70365: f64, t70369: f64, t70373: f64, t15675: f64, t4965: f64, t70381: f64, t76079: f64, t76087: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78474 = 0.20455996240684006298e-1_f64 * t76050;
    let t78475 = 0.20455996240684006298e-1_f64 * t76054;
    let t78476 = 0.86737941314158990619e-4_f64 * t70358;
    let t78477 = 0.14967802127329760705e-1_f64 * t76066;
    let t78478 = 0.30487649791575028312e-3_f64 * t70365;
    let t78479 = 0.43368970657079495308e-4_f64 * t70369;
    let t78480 = 0.30487649791575028312e-3_f64 * t70373;
    let t78482 = 0.11974241701863808564e0_f64 * t4965 * t15675;
    let t78483 = 0.16263363996404810741e-4_f64 * t70381;
    let t78484 = 0.13637330827122670865e-1_f64 * t76079;
    let t78486 = 0.5107751987195740728e-4_f64 * t76087;
    (t78474, t78475, t78476, t78477, t78478, t78479, t78480, t78482, t78483, t78484, t78486)
}
