//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1254/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1254(t20154: f64, t219: f64, t6420: f64, t1265: f64, t18490: f64, t6424: f64, t18967: f64, t19521: f64, t1266: f64, t1657: f64, t1842: f64, t18483: f64, t18496: f64, t18950: f64, t19507: f64, t19509: f64, t4494: f64, t4517: f64, t538: f64, t5739: f64, t5921: f64, t5925: f64, t5930: f64, t5933: f64, t6260: f64, t6425: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t20155 = param_beta * t20154;
    let t20157 = t6420 * t219;
    let t20171 = t18490 * t6424 * t1265;
    let t20174 = t18967 * t19521;
    let t20177 = -t1266 * t20157 - t1657 * t18950 - t1842 * t19507 + 2.0_f64 * t18483 * t6425 - 2.0_f64 * t18496 * t20174 + 2.0_f64 * t19509 * t5925 + t19509 * t5930 + t20155 * t538 - 6.0_f64 * t20171 * t5739 + 2.0_f64 * t4494 * t5921 - t4517 * t5921 - t5933 * t6260;
    (t20155, t20157, t20171, t20174, t20177)
}
