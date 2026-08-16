//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 848/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk848(t1359: f64, t2435: f64, t555: f64, t785: f64, t1358: f64, t2439: f64, t1419: f64, t212: f64, t689: f64, t1357: f64, t1445: f64, t2453: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3894 = 0.73171657588172351096e-2_f64 * t2435 * t1359;
    let t3895 = t785 * t555;
    let t3896 = t3895 * t1358;
    let t3898 = 0.65049603595885220126e-3_f64 * t2439 * t3896;
    let t3899 = t212 * t1419;
    let t3900 = t3899 * t1358;
    let t3901 = t689 * t3900;
    let t3903 = t1357 * t1445;
    let t3904 = t689 * t3903;
    let t3906 = t2453 * t556;
    (t3894, t3895, t3896, t3898, t3899, t3900, t3901, t3903, t3904, t3906)
}
