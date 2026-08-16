//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2943/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2943(t1011: f64, t4886: f64, t697: f64, t1065: f64, t372: f64, t4866: f64, t11774: f64, t16103: f64, t42254: f64, t42257: f64, t42259: f64, t42268: f64, t42270: f64, t42274: f64, t42282: f64, t42284: f64, t42288: f64) -> (f64, f64) {
    let t53542 = t1011 * t697 * t4886;
    let t53543 = t53542 / 432.0_f64;
    let t53545 = t372 * t1065 * t4866;
    let t53549 = -t42254 / 432.0_f64 - t42257 / 324.0_f64 + 0.85748036236139473944e-3_f64 * t42259 - 0.42874018118069736972e-3_f64 * t42268 - 0.15244095330869239812e-2_f64 * t42270 - 0.19055119163586549765e-3_f64 * t42274 + 0.45732285992607719436e-2_f64 * t42282 - 0.42874018118069736972e-3_f64 * t42284 - 0.14291339372689912324e-3_f64 * t42288 - t53543 - 0.85748036236139473944e-3_f64 * t11774 * t53545 * t16103;
    (t53545, t53549)
}
