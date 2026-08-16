//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1202/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1202(t1780: f64, t932: f64, t143: f64, t19107: f64, t346: f64, t10631: f64, t1143: f64, t14907: f64, t14966: f64, t14996: f64, t15046: f64, t18542: f64, t18555: f64, t18596: f64, t18600: f64, t18753: f64, t18788: f64, t3381: f64, t348: f64, t4602: f64, t4607: f64, t4638: f64, t4643: f64, t4671: f64, t5111: f64, t5122: f64, t6589: f64) -> f64 {
    let t20098 = t1780 * t932;
    let t20107 = t19107 * t143;
    let t20112 = t1780 * t346;
    let t20126 = 0.11791604938271604938e-1_f64 * t14966 - 0.9286875e-2_f64 * t3381 * t18596 + 0.17687407407407407407e-1_f64 * t14996 + 0.9286875e-2_f64 * t20098 * t4602 - 0.1857375e-1_f64 * t3381 * t18542 + 0.46434375e-2_f64 * t5111 * t18788 + 0.123825e-1_f64 * t5122 * t18600 + 0.619125e-2_f64 * t20107 * t348 + 0.24765e-1_f64 * t5122 * t18555 + 0.24765e-1_f64 * t20112 * t4643 - 0.1857375e-1_f64 * t14907 * t4638 - 0.1857375e-1_f64 * t3381 * t18753 + 0.88437037037037037037e-2_f64 * t10631 - t15046 - 0.123825e-1_f64 * t1780 * t4671 + 0.46434375e-2_f64 * t1143 * t6589 - 0.1857375e-1_f64 * t14907 * t4607;
    t20126
}
