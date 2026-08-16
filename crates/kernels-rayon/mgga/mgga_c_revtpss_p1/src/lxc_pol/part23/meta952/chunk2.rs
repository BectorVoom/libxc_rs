//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3157/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3157(t17569: f64, t20783: f64, t1042: f64, t1261: f64, t12866: f64, t17693: f64, t17694: f64, t20820: f64, t5268: f64, t5287: f64, t69936: f64, t69939: f64, t69947: f64, t69961: f64, t69964: f64, t69966: f64, t78770: f64, t82587: f64, t82591: f64) -> f64 {
    let t82932 = t17569 * t20783;
    let t82950 = 0.28582678745379824648e-3_f64 * t69936 + 0.57165357490759649295e-3_f64 * t69939 + 0.57165357490759649296e-3_f64 * t82932 - 0.30488190661738479624e-2_f64 * t69947 - 0.7145669686344956162e-3_f64 * t12866 * t17694 * t82591 + 0.71456696863449561621e-3_f64 * t17693 * t17694 * t82587 + 0.64311027177104605458e-3_f64 * t20820 * t5287 - 0.5081365110289746604e-2_f64 * t69961 + 0.14291339372689912324e-3_f64 * t69964 + 0.57165357490759649296e-3_f64 * t69966 - 0.28582678745379824648e-3_f64 * t1261 * t1042 * t5268 * t78770;
    t82950
}
