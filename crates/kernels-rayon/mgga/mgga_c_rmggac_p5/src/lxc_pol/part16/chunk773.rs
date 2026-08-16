//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 773/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk773(t1997: f64, t36520: f64, t7335: f64, t7927: f64, t3924: f64, t504: f64, t2185: f64, t7407: f64, t507: f64, t8629: f64, t124: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36521 = t36520 * t1997;
    let t36527 = t7335 * t7927;
    let t36596 = t504 * t3924;
    let t36612 = t7407 * t2185;
    let t36629 = t507 * t8629;
    let t36632 = t124 * t338;
    (t36521, t36527, t36596, t36612, t36629, t36632)
}
