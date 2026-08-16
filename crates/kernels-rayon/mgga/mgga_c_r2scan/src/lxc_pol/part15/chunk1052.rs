//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1052/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1052(t37412: f64, t10972: f64, t37365: f64, t10831: f64, t1102: f64, t3457: f64, t2312: f64, t597: f64, t10680: f64, t10682: f64, t10645: f64, t10646: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37413 = 0.91462949374725084942e-3_f64 * t37412;
    let t37414 = t37365 * t10972;
    let t37415 = 0.91462949374725084942e-3_f64 * t37414;
    let t37419 = t1102 * t10831 * t3457;
    let t37421 = t597 * t2312;
    let t37423 = t10680 * t10682 * t37421;
    let t37426 = t10645 * t10646 * t550;
    (t37413, t37415, t37419, t37421, t37423, t37426)
}
