//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 629/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk629(t1356: f64, t8258: f64, t8278: f64, t884: f64, t2265: f64, t942: f64, t2416: f64, t7487: f64, t2160: f64, t2339: f64, t638: f64, t2323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8306 = t1356 * t8258;
    let t8307 = 0.39914139006212695214e-1_f64 * t8306;
    let t8308 = t884 * t8278;
    let t8309 = 0.59871208509319042821e-1_f64 * t8308;
    let t8310 = t942 * t2265;
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    (t8307, t8309, t8310, t8328, t8331, t8334)
}
