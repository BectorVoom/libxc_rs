//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1928/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1928(t26426: f64, t81046: f64, t22690: f64, t7732: f64, t81195: f64, t16413: f64, t1985: f64, t1998: f64, t214: f64, t16248: f64, t22833: f64, t16383: f64) -> (f64, f64, f64, f64, f64) {
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    let t91091 = t1985 * t214 * t1998 * t16413;
    let t91094 = t22833 * t16248;
    let t91096 = t22833 * t16383;
    (t91078, t91081, t91091, t91094, t91096)
}
