//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 647/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk647(t837: f64, t899: f64, t507: f64, t7190: f64, t2144: f64, t7262: f64) -> (f64, f64, f64, f64) {
    let t26144 = t899 * t837;
    let t26283 = t507 * t7190;
    let t26287 = t899 * t2144;
    let t26291 = t507 * t7262;
    (t26144, t26283, t26287, t26291)
}
