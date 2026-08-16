//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 812/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk812(t38350: f64, t7473: f64, t34884: f64, t9046: f64, t2289: f64, t34881: f64, t16501: f64, t7363: f64, t1966: f64, t1180: f64, t34759: f64, t338: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39832 = t38350 * t7473;
    let t39840 = t34884 * t9046;
    let t39842 = t34881 * t2289;
    let t39850 = t7363 * t16501;
    let t39851 = t1966 * t39850;
    let t39857 = t1180 * t34759;
    let t39866 = t338 * t615;
    (t39832, t39840, t39842, t39850, t39851, t39857, t39866)
}
