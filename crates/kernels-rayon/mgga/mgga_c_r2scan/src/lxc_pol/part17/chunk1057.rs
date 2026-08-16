//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1057/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1057(t10647: f64, t10649: f64, t2049: f64, t3438: f64, t357: f64, t6806: f64, t10972: f64, t37365: f64, t10831: f64, t1102: f64, t3457: f64, t2312: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t37412 = t6806 * t357 * t10647 * t10649 * t3438 * t2049;
    let t37414 = t37365 * t10972;
    let t37419 = t1102 * t10831 * t3457;
    let t37421 = t597 * t2312;
    (t37412, t37414, t37419, t37421)
}
