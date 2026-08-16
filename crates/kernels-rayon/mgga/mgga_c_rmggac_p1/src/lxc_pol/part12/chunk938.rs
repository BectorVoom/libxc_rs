//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 938/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk938(t2392: f64, t833: f64, t262: f64, t7204: f64, t5058: f64, t511: f64, t7284: f64, t34975: f64, t34976: f64, t571: f64, t7455: f64, t39850: f64, t7229: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40134 = t2392 * t833;
    let t40135 = t262 * t40134;
    let t40136 = t7204 * t40135;
    let t40138 = t5058 * t511;
    let t40139 = t40138 * t7284;
    let t40143 = t34975 * t34976 * t571 * t7455;
    let t40145 = t7229 * t39850;
    (t40134, t40135, t40136, t40139, t40143, t40145)
}
