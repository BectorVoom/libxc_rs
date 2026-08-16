//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1098/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1098(t2056: f64, t34258: f64, t7741: f64, t94: f64, t7978: f64, t8634: f64, t5542: f64, t8714: f64, t2014: f64, t7898: f64, t8718: f64, t196: f64, t197: f64, t8075: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34260 = 2.0_f64 * t34258 * t2056;
    let t34261 = t94 * t7741;
    let t34263 = 2.0_f64 * t34261 * t2056;
    let t34265 = 2.0_f64 * t8634 * t7978;
    let t34266 = t8714 * t5542;
    let t34267 = t2014 * t34266;
    let t34268 = t7898 * t8718;
    let t34270 = t8075 * t196 * t197;
    (t34260, t34261, t34263, t34265, t34266, t34267, t34268, t34270)
}
