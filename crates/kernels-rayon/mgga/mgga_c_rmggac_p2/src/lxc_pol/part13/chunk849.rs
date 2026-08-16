//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 849/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk849(t7269: f64, t8368: f64, t7494: f64, t8537: f64, t1587: f64, t2084: f64, t2134: f64, t27: f64, t7512: f64, t36471: f64, t5145: f64, t656: f64) -> (f64, f64, f64, f64, f64) {
    let t39023 = t8368 * t7269;
    let t39025 = t7494 * t8537;
    let t39031 = t2134 * t27 * t2084 * t1587;
    let t39033 = t8368 * t7512;
    let t39036 = t36471 * t656 * t5145;
    (t39023, t39025, t39031, t39033, t39036)
}
