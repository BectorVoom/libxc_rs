//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1329/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1329(t3172: f64, t4868: f64, t1041: f64, t3168: f64, t4878: f64, t11150: f64, t3181: f64, t11144: f64, t11852: f64, t3124: f64, t4820: f64, t1655: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16163 = t3172 * t4868;
    let t16165 = 0.28582678745379824648e-3_f64 * t1041 * t16163;
    let t16190 = t4878 * t3168;
    let t16199 = t3181 * t11150;
    let t16208 = t11852 * t11144;
    let t16218 = 0.28582678745379824648e-3_f64 * t3124 * t4820;
    let t16219 = t697 * t1655;
    (t16165, t16190, t16199, t16208, t16218, t16219)
}
