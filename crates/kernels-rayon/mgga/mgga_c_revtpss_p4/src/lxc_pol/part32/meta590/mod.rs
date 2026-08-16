//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1920;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta590(t28399: f64, t686: f64, t72: f64, t7058: f64, t103000: f64, t93371: f64, t25410: f64, t8011: f64, t93240: f64, t7064: f64, t28447: f64, t689: f64, t887: f64, t26485: f64, t99463: f64, t102986: f64, t25387: f64, t1580: f64, t2439: f64, t26434: f64, t2453: f64, t2458: f64, t7998: f64, t41040: f64, t685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103119, t103122, t103130, t103136, t103140) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1920(t28399, t686, t72, t7058, t103000, t93371, t25410, t8011, t93240, t7064, t28447, t689, t887);
        let (t103142, t103156, t103158, t103161, t103181) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1921(t26485, t99463, t102986, t25387, t1580, t2439, t26434, t2453, t2458, t7998, t41040, t685);
    (t103119, t103122, t103130, t103136, t103140, t103142, t103156, t103158, t103161, t103181)
}
