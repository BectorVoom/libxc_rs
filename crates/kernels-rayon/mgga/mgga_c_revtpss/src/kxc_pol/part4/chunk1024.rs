//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1024/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1024(t10994: f64, t786: f64, t2771: f64, t676: f64, t123: f64, t2435: f64, t2448: f64, t2440: f64, t887: f64, t2439: f64, t866: f64, t225: f64) -> (f64, f64, f64, f64, f64) {
    let t10995 = t786 * t10994;
    let t10996 = t676 * t2771;
    let t10997 = t123 * t10996;
    let t10998 = t10995 * t10997;
    let t11000 = t2435 * t2448;
    let t11003 = t2440 * t887;
    let t11004 = t2439 * t11003;
    let t11006 = t866 * t866;
    let t11007 = 1.0_f64 / t11006;
    let t11008 = t225 * t11007;
    (t10995, t10998, t11000, t11004, t11008)
}
