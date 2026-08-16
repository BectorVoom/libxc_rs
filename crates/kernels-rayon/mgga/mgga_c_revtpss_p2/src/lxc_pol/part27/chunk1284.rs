//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1284/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1284(t2014: f64, t2034: f64, t46304: f64, t1936: f64, t46126: f64, t49851: f64, t10416: f64, t7002: f64, t49693: f64, t13435: f64, t2322: f64, t25832: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94944 = t2014 * t2034 * t46304;
    let t94956 = 2.0_f64 * t46126 * t1936;
    let t94958 = 6.0_f64 * t49851 * t1936;
    let t94960 = 6.0_f64 * t10416 * t7002;
    let t94962 = 6.0_f64 * t49693 * t1936;
    let t94964 = 12.0_f64 * t13435 * t7002;
    let t94966 = 6.0_f64 * t2322 * t25832;
    (t94944, t94956, t94958, t94960, t94962, t94964, t94966)
}
