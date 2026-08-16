//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 863/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk863(t1587: f64, t2084: f64, t2134: f64, t27: f64, t7512: f64, t8368: f64, t36471: f64, t5145: f64, t656: f64, t34938: f64, t5149: f64, t1550: f64, t2060: f64, t27059: f64) -> (f64, f64, f64, f64, f64) {
    let t39031 = t2134 * t27 * t2084 * t1587;
    let t39033 = t8368 * t7512;
    let t39036 = t36471 * t656 * t5145;
    let t39039 = t34938 * t656 * t5149;
    let t39042 = t1550 * t2060 * t27059;
    (t39031, t39033, t39036, t39039, t39042)
}
