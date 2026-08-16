//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 869/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk869(t28528: f64, t587: f64, t1674: f64, t22750: f64, t2396: f64, t28461: f64, t28464: f64, t28467: f64, t28470: f64, t28472: f64, t28476: f64, t28509: f64, t6851: f64, t8609: f64, t8613: f64) -> (f64, f64) {
    let t28530 = 0.62182e-1_f64 * t28528 * t587;
    let t28531 = -t28461 + t28464 - t28467 + t28470 - 0.1025389702100779493e4_f64 * t1674 * t28472 + 0.1038945353962551798e3_f64 * t1674 * t28476 - 0.58482233974552040708e0_f64 * t1674 * t28509 - 0.17544670192365612213e1_f64 * t22750 * t2396 - 0.17544670192365612213e1_f64 * t6851 * t8609 - 0.51947267698127589899e2_f64 * t6851 * t8613 - t28530;
    (t28530, t28531)
}
