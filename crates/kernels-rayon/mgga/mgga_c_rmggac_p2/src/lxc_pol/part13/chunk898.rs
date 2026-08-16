//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 898/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk898(t2347: f64, t833: f64, t262: f64, t8640: f64, t848: f64, t7198: f64, t1165: f64, t1979: f64, t1982: f64, t201: f64, t589: f64, t2410: f64, t4443: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39875 = t2347 * t833;
    let t39876 = t262 * t39875;
    let t39877 = t8640 * t39876;
    let t39879 = t2347 * t848;
    let t39880 = t262 * t39879;
    let t39881 = t7198 * t39880;
    let t39889 = t589 * t1165 * t201 * t1979 * t1982;
    let t39892 = t2410 * t4443 * t674;
    (t39875, t39876, t39877, t39879, t39880, t39881, t39889, t39892)
}
