//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1200/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1200<F: Float>(t14171: F, t4606: F, t10556: F, t1143: F, t1153: F, t14913: F, t14926: F, t14927: F, t18686: F, t18745: F, t18749: F, t18793: F, t20020: F, t20024: F, t20028: F, t20031: F, t20034: F, t3381: F, t365: F, t5111: F, t6597: F, t6601: F) -> F {
    let t20037 = t4606 * t14171;
    let t20055 = F::cast_from(0.53062222222222222222e-1_f64) * t1153 * t20020 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t20024 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t20028 - F::cast_from(0.17687407407407407407e-1_f64) * t20031 + F::cast_from(0.371475e-1_f64) * t3381 * t20034 - F::cast_from(0.9286875e-2_f64) * t5111 * t20037 - F::cast_from(0.35374814814814814815e-1_f64) * t14913 - t14926 - F::cast_from(0.70749629629629629628e-1_f64) * t14927 - F::cast_from(0.46434375e-2_f64) * t5111 * t18745 + F::cast_from(0.9286875e-2_f64) * t5111 * t18749 + F::cast_from(0.58958024691358024691e-2_f64) * t10556 - F::cast_from(0.9286875e-2_f64) * t3381 * t18793 + F::cast_from(0.9286875e-2_f64) * t1143 * t6597 + F::cast_from(0.9286875e-2_f64) * t365 * t18686 + F::cast_from(0.123825e-1_f64) * t1143 * t6601;
    t20055
}
