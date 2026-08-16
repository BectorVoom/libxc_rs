//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 190/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk190(t27: f64, t649: f64, t648: f64, t305: f64, t36: f64, t22: f64, t326: f64, t262: f64) -> (f64, f64, f64, f64, f64) {
    let t650 = t27 * t649;
    let t651 = t648 * t650;
    let t653 = t305 * t36;
    let t655 = t326 * t22;
    let t656 = t262 * t36;
    (t650, t651, t653, t655, t656)
}
