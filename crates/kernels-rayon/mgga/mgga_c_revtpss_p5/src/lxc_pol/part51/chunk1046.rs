//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1046/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1046(t31991: f64, t93962: f64, t1071: f64, t31902: f64, t1039: f64, t246: f64, t11874: f64, t373: f64, t385: f64, t372: f64, t3316: f64, t7150: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120297 = t93962 * t31991;
    let t120301 = t31902 * t1071 * t31991;
    let t120304 = t1039 * t246;
    let t120305 = t11874 * t120304;
    let t120306 = t373 * t385;
    let t120307 = t372 * t120306;
    let t120313 = t7150 * t3316 * t120304;
    (t120297, t120301, t120304, t120305, t120306, t120307, t120313)
}
