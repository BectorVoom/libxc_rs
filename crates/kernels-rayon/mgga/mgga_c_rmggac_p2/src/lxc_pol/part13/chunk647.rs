//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 647/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk647(t1652: f64, t649: f64, t27: f64, t2145: f64, t674: f64, t8450: f64) -> (f64, f64, f64) {
    let t8567 = t649 * t1652;
    let t8568 = t27 * t8567;
    let t8569 = t2145 * t8568;
    let t8571 = t8450 * t674;
    (t8568, t8569, t8571)
}
