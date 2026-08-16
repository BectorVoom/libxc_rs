//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1371/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1371(t15216: f64, t28101: f64, t26960: f64, t1268: f64, t9494: f64, t26955: f64, t13132: f64, t13150: f64, t28102: f64, t28116: f64, t5302: f64, t5310: f64, t7772: f64, t92795: f64, t93099: f64, t93134: f64, t96247: f64, t96251: f64, t96831: f64, t97019: f64) -> f64 {
    let t97330 = t15216 * t28101;
    let t97332 = 0.7722800925925925926e-4_f64 * t26960 * t97330;
    let t97338 = t1268 * t9494;
    let t97344 = 0.10306077835648148148e-4_f64 * t26955 * t97330;
    let t97347 = -0.30918233506944444444e-4_f64 * t26955 * t97019 + 0.46377350260416666667e-4_f64 * t7772 * t96831 + 0.61782407407407407408e-3_f64 * t93099 - 0.61782407407407407408e-3_f64 * t92795 * t28102 + t97332 + 0.23168402777777777778e-3_f64 * t26960 * t5310 * t28116 * t13150 + 0.7722800925925925926e-4_f64 * t93134 + 0.92673611111111111112e-3_f64 * t26960 * t5302 * t97338 * t13132 + t97344 - 0.15476481481481481481e-2_f64 * t96247 - 0.51588271604938271604e-3_f64 * t96251;
    t97347
}
