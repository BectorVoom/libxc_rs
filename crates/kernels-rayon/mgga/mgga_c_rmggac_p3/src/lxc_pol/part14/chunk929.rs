//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 929/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk929(t36978: f64, t5169: f64, t656: f64, t34738: f64, t5260: f64, t36471: f64, t5263: f64, t1550: f64, t2060: f64, t29892: f64, t27044: f64, t903: f64) -> (f64, f64, f64, f64, f64) {
    let t40012 = t36978 * t656 * t5169;
    let t40015 = t34738 * t656 * t5260;
    let t40018 = t36471 * t656 * t5263;
    let t40021 = t1550 * t2060 * t29892;
    let t40024 = t903 * t2060 * t27044;
    (t40012, t40015, t40018, t40021, t40024)
}
