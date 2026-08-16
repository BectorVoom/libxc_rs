//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 983/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk983(t12599: f64, t1277: f64, t1204: f64, t1269: f64, t1294: f64, t3584: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64, f64) {
    let t12600 = t1277 * t12599;
    let t12603 = t1204 * t1269;
    let t12606 = t3584 * t1294;
    let t12607 = t1277 * t12606;
    let t12610 = 0.46096296296296296297e-1_f64 * t12295;
    let t12621 = -t12610 + 0.19755555555555555556e-1_f64 * t12297 + 0.9877777777777777778e-2_f64 * t12299 - 0.29633333333333333334e-1_f64 * t12301 - 0.14816666666666666667e-1_f64 * t12303 + 0.16462962962962962963e-1_f64 * t12307 - 0.59266666666666666668e-1_f64 * t12310 - 0.29633333333333333334e-1_f64 * t12292 + 0.88900000000000000002e-1_f64 * t12314 + 0.88900000000000000002e-1_f64 * t12317 + 0.14816666666666666667e-1_f64 * t12320;
    (t12600, t12603, t12607, t12621)
}
