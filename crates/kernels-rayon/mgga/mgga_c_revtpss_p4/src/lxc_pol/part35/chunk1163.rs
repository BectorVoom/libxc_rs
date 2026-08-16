//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1163/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1163(t30266: f64, t689: f64, t25904: f64, t109412: f64, t25878: f64, t109403: f64, t94669: f64, t30308: f64, t686: f64, t72: f64, t25895: f64, t25899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109425 = t30266 * t689;
    let t109426 = t25904 * t109425;
    let t109434 = t25878 * t109412;
    let t109437 = t94669 * t109403;
    let t109449 = t30308 * t72 * t686;
    let t109450 = t25895 * t109449;
    let t109453 = t25878 * t109449;
    let t109455 = t25899 * t109425;
    (t109426, t109434, t109437, t109450, t109453, t109455)
}
