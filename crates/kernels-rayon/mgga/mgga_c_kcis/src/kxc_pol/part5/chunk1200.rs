//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1200/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1200(t14171: f64, t4606: f64, t10556: f64, t1143: f64, t1153: f64, t14913: f64, t14926: f64, t14927: f64, t18686: f64, t18745: f64, t18749: f64, t18793: f64, t20020: f64, t20024: f64, t20028: f64, t20031: f64, t20034: f64, t3381: f64, t365: f64, t5111: f64, t6597: f64, t6601: f64) -> f64 {
    let t20037 = t4606 * t14171;
    let t20055 = 0.53062222222222222222e-1_f64 * t1153 * t20020 - 0.26531111111111111111e-1_f64 * t1153 * t20024 - 0.26531111111111111111e-1_f64 * t1153 * t20028 - 0.17687407407407407407e-1_f64 * t20031 + 0.371475e-1_f64 * t3381 * t20034 - 0.9286875e-2_f64 * t5111 * t20037 - 0.35374814814814814815e-1_f64 * t14913 - t14926 - 0.70749629629629629628e-1_f64 * t14927 - 0.46434375e-2_f64 * t5111 * t18745 + 0.9286875e-2_f64 * t5111 * t18749 + 0.58958024691358024691e-2_f64 * t10556 - 0.9286875e-2_f64 * t3381 * t18793 + 0.9286875e-2_f64 * t1143 * t6597 + 0.9286875e-2_f64 * t365 * t18686 + 0.123825e-1_f64 * t1143 * t6601;
    t20055
}
