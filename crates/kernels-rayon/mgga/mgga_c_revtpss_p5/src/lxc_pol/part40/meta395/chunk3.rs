//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1434/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1434(t3368: f64, t5277: f64, t1042: f64, t3704: f64, t5274: f64, t1774: f64, t3588: f64, t1250: f64, t3720: f64, t1285: f64, t17395: f64, t1032: f64, t5216: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17588 = t5277 * t3368;
    let t17589 = t1042 * t17588;
    let t17593 = 0.28582678745379824648e-3_f64 * t5274 * t3704;
    let t17600 = t1774 * t3588;
    let t17601 = t17600 * t1250;
    let t17602 = t3720 * t17601;
    let t17605 = t1285 * t17395;
    let t17608 = t5216 * t1032;
    (t17589, t17593, t17600, t17602, t17605, t17608)
}
