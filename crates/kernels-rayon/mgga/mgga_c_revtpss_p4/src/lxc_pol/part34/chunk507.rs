//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 507/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk507(t2630: f64, t3869: f64, t1337: f64, t2619: f64, t514: f64, t517: f64, t1359: f64, t2435: f64, t555: f64, t785: f64, t1358: f64, t2439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3871 = 0.10843581300301739842e-1_f64 * t3869 * t2630;
    let t3873 = 0.24415263074675393405e-3_f64 * t1337 * t2619;
    let t3874 = 1.0_f64 / t514;
    let t3881 = 1.0_f64 / t517;
    let t3894 = 0.73171657588172351096e-2_f64 * t2435 * t1359;
    let t3895 = t785 * t555;
    let t3896 = t3895 * t1358;
    let t3898 = 0.65049603595885220126e-3_f64 * t2439 * t3896;
    (t3871, t3873, t3874, t3881, t3894, t3895, t3896, t3898)
}
