//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 926/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk926(t8507: f64, t999: f64, t31892: f64, t1071: f64, t8513: f64, t8521: f64, t1032: f64, t994: f64, t8501: f64) -> (f64, f64, f64, f64, f64) {
    let t31904 = t8507 * t999;
    let t31905 = t31892 * t31904;
    let t31908 = t8513 * t1071;
    let t31909 = t31908 * t8521;
    let t31912 = t994 * t1032;
    let t31913 = t31912 * t8501;
    (t31905, t31908, t31909, t31912, t31913)
}
