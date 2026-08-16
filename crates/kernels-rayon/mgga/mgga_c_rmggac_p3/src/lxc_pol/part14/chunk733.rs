//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 733/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk733(t7279: f64, t7501: f64, t2084: f64, t2139: f64, t27: f64, t848: f64, t2189: f64, t7228: f64, t3350: f64) -> (f64, f64, f64, f64) {
    let t34822 = t7501 * t7279;
    let t34826 = t2139 * t27 * t2084 * t848;
    let t34846 = t2189 * t7228;
    let t34847 = t34846 * t3350;
    (t34822, t34826, t34846, t34847)
}
