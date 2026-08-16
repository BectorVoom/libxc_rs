//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1746/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1746(t233: f64, t28340: f64, t1957: f64, t2061: f64, t231: f64, t4423: f64, t7076: f64, t25317: f64, t8006: f64, t886: f64, t4533: f64, t7071: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28399 = t233 * t28340;
    let t28400 = t1957 * t28399;
    let t28404 = t2061 * t4423 * t231;
    let t28405 = t7076 * t28404;
    let t28411 = t25317 * t8006 * t886;
    let t28417 = t2061 * t4533;
    let t28418 = t7071 * t28417;
    (t28399, t28400, t28404, t28405, t28411, t28417, t28418)
}
