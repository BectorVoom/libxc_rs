//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1036/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1036(t1401: f64, t32284: f64, t1405: f64, t8591: f64, t1412: f64, t241: f64, t125: f64, t1353: f64, t246: f64, t1459: f64, t8614: f64, t116: f64, t8460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32285 = t32284 * t1401;
    let t32287 = t8591 * t1405;
    let t32289 = t241 * t1412;
    let t32291 = t246 * t125 * t1353;
    let t32292 = t32289 * t32291;
    let t32293 = t8591 * t32292;
    let t32372 = t1459 * t8614;
    let t32373 = 3.0_f64 * t32372;
    let t32374 = t116 * t8460;
    (t32285, t32287, t32289, t32291, t32292, t32293, t32373, t32374)
}
