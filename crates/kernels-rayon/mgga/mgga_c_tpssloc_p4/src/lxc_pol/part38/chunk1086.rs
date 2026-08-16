//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1086/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1086(t13532: f64, t2826: f64, t136: f64, t10216: f64, t1409: f64, t2244: f64, t10304: f64, t2775: f64, t3966: f64, t607: f64, t908: f64, t2250: f64, t4342: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13533 = t2826 * t13532;
    let t13534 = t136 * t13533;
    let t13536 = t10216 * t1409;
    let t13537 = t13536 * t2244;
    let t13538 = t10304 * t13537;
    let t13539 = t136 * t13538;
    let t13541 = t2775 * t3966;
    let t13542 = t13541 * t607;
    let t13543 = t908 * t13542;
    let t13544 = t136 * t13543;
    let t13546 = t4342 * t2250;
    (t13534, t13537, t13539, t13542, t13544, t13546)
}
