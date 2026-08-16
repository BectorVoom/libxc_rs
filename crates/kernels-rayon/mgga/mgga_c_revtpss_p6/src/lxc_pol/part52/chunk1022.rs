//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1022/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1022(t31846: f64, t839: f64, t846: f64, t8486: f64, t241: f64, t853: f64, t125: f64, t246: f64, t775: f64, t30: f64, t7086: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31847 = t31846 * t839;
    let t31849 = t8486 * t846;
    let t31851 = t241 * t853;
    let t31853 = t246 * t125 * t775;
    let t31854 = t31851 * t31853;
    let t31855 = t8486 * t31854;
    let t31873 = t30 * t7086;
    let t32080 = t33 * t7086;
    (t31847, t31849, t31851, t31853, t31854, t31855, t31873, t32080)
}
