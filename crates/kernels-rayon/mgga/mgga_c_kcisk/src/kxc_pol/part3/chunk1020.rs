//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1020/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1020(t14920: f64, t14959: f64, t15004: f64, t15072: f64, t1597: f64, t14230: f64, t14232: f64, t14237: f64, t14247: f64, t14250: f64, t14253: f64, t14258: f64, t14262: f64, t14268: f64, t14271: f64, t14866: f64, t1557: f64, t548: f64) -> f64 {
    let t15074 = t14920 + t14959 + t15004 + t15072;
    let t15075 = t15074 * t1597;
    let t15079 = -0.46429444444444444443e-2_f64 * t14230 - 0.12381185185185185185e-1_f64 * t14232 + 0.69644166666666666665e-2_f64 * t14237 + 0.34048259259259259259e-1_f64 * t14247 + t14866 * t548 + 0.30952962962962962963e-2_f64 * t14250 + 0.51072388888888888887e-1_f64 * t14253 + 0.38691203703703703703e-2_f64 * t14258 - 0.77382407407407407405e-3_f64 * t14262 + 0.69644166666666666665e-2_f64 * t14268 - 0.193e0_f64 * t1557 * t15075 - 0.13928833333333333333e-1_f64 * t14271;
    t15079
}
