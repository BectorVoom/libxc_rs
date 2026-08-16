//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 909/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk909(t34944: f64, t6558: f64, t656: f64, t34738: f64, t6561: f64, t36471: f64, t6564: f64, t34938: f64, t6523: f64, t8526: f64, t8659: f64, t2085: f64, t9762: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45126 = t34944 * t656 * t6558;
    let t45129 = t34738 * t656 * t6561;
    let t45132 = t36471 * t656 * t6564;
    let t45135 = t34938 * t656 * t6523;
    let t45139 = t8659 * t8526;
    let t45149 = t9762 * t2085;
    (t45126, t45129, t45132, t45135, t45139, t45149)
}
