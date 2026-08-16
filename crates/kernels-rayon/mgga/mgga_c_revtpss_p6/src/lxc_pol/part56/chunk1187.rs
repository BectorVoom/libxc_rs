//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1187/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1187(t33521: f64, t34972: f64, t1794: f64, t3596: f64, t1042: f64, t1122: f64, t1214: f64, t124554: f64, t124578: f64, t124594: f64, t124604: f64, t124611: f64, t124613: f64, t124744: f64, t124869: f64, t1263: f64, t131497: f64, t131608: f64, t1769: f64, t2148: f64, t247: f64, t26969: f64, t3153: f64, t33398: f64, t33461: f64, t33462: f64, t33469: f64, t33478: f64, t34901: f64, t34960: f64, t3719: f64, t494: f64, t5215: f64, t5270: f64, t5296: f64, t5351: f64, t5402: f64, t5428: f64, t5465: f64, t7627: f64, t8197: f64, t96928: f64) -> f64 {
    let t131799 = t34972 * t33521;
    let t131810 = t3596 * t1794;
    let t131815 = -0.28234466758480466999e-3_f64 * t124611 * t124613 * t5351 * t96928 + 0.24791552806034007214e-3_f64 * t131608 * t5270 + 0.3718732920905101082e-3_f64 * t124578 * t1042 * t1263 * t1769 * t1122 - 0.24791552806034007214e-3_f64 * t124594 * t1042 * t5296 * t131497 + 0.24791552806034007213e-3_f64 * t124744 * t5402 + 0.3427184259906141157e1_f64 * t33461 * t33462 * t8197 * t7627 - 0.52041769129231196772e1_f64 * t2148 * t124604 * t26969 * t5428 + 0.3718732920905101082e-3_f64 * t124554 * t34901 - 0.12395776403017003607e-3_f64 * t131799 + 0.56468933516960933998e-3_f64 * t33398 * t247 * t3719 * t494 * t5215 + 0.51407763898592117355e1_f64 * t33469 * t33478 * t34960 * t1214 - 0.17347256376410398924e1_f64 * t124869 * t131810 * t3153 * t5465;
    t131815
}
