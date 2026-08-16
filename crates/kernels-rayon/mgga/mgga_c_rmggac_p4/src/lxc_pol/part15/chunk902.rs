//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 902/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk902(t36471: f64, t656: f64, t6583: f64, t36634: f64, t6586: f64, t34944: f64, t6558: f64, t34738: f64, t6561: f64, t6564: f64, t34938: f64, t6523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45120 = t36471 * t656 * t6583;
    let t45123 = t36634 * t656 * t6586;
    let t45126 = t34944 * t656 * t6558;
    let t45129 = t34738 * t656 * t6561;
    let t45132 = t36471 * t656 * t6564;
    let t45135 = t34938 * t656 * t6523;
    (t45120, t45123, t45126, t45129, t45132, t45135)
}
