//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1075/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1075<F: Float>(t10560: F, t1154: F, t6272: F, t3405: F, t6276: F, t1155: F, t18443: F, t3393: F, t6673: F, t1045: F, t1727: F, t14215: F, t14171: F, t4606: F, t10556: F, t1143: F, t1153: F, t14913: F, t14926: F, t14927: F, t18686: F, t18745: F, t18749: F, t18793: F, t3381: F, t365: F, t5111: F, t6597: F, t6601: F) -> (F,) {
    let t20020 = t1154 * t10560 * t6272;
    let t20024 = t1154 * t3405 * t6276;
    let t20028 = t1154 * t1155 * t18443;
    let t20031 = t3393 * t6673;
    let t20033 = t1727 * t1045;
    let t20034 = t14215 * t20033;
    let t20037 = t4606 * t14171;
    let t20055 = 0.53062222222222222222e-1 * t1153 * t20020 - 0.26531111111111111111e-1 * t1153 * t20024 - 0.26531111111111111111e-1 * t1153 * t20028 - 0.17687407407407407407e-1 * t20031 + 0.371475e-1 * t3381 * t20034 - 0.9286875e-2 * t5111 * t20037 - 0.35374814814814814815e-1 * t14913 - t14926 - 0.70749629629629629628e-1 * t14927 - 0.46434375e-2 * t5111 * t18745 + 0.9286875e-2 * t5111 * t18749 + 0.58958024691358024691e-2 * t10556 - 0.9286875e-2 * t3381 * t18793 + 0.9286875e-2 * t1143 * t6597 + 0.9286875e-2 * t365 * t18686 + 0.123825e-1 * t1143 * t6601;
    (t20055,)
}
