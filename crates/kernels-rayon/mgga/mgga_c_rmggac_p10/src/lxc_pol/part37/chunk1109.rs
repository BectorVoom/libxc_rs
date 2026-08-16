//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1109/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1109(t118: f64, t338: f64, t76326: f64, t78046: f64, t78048: f64, t78050: f64, t78051: f64, t78053: f64, t78055: f64, t78060: f64, t78061: f64, t78062: f64, t78065: f64, t78067: f64, t80372: f64) -> f64 {
    let t80477 = 0.19957069503106347607e-1_f64 * t118 * t338 * t80372 + t78046 - t78048 - t78050 - t78051 + t76326 + t78053 + t78055 - t78060 + t78061 + t78062 + t78065 - t78067;
    t80477
}
