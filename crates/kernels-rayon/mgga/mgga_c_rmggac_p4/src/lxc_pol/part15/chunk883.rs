//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 883/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk883(t2010: f64, t2415: f64, t4962: f64, t5002: f64, t7487: f64, t9723: f64, t2011: f64, t291: f64, t5878: f64, t935: f64, t9719: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44825 = t2010 * t2415 * t4962;
    let t44828 = t2010 * t2415 * t5002;
    let t44831 = t7487 * t9723;
    let t44835 = t2010 * t2011 * t5878 * t291;
    let t44838 = t2010 * t9719 * t935;
    let t44841 = t2010 * t9719 * t938;
    (t44825, t44828, t44831, t44835, t44838, t44841)
}
