//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 910/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk910(t29146: f64, t29172: f64, t1737: f64, t1746: f64, t10902: f64, t29102: f64, t10906: f64, t1735: f64, t23709: f64, t2432: f64, t29074: f64, t29099: f64, t29104: f64, t29108: f64, t29112: f64, t621: f64, t7151: f64, t8748: f64, t8765: f64, t8769: f64) -> f64 {
    let t29173 = t29146 + t29172;
    let t29175 = t1737 * t29173 * t1746;
    let t29182 = t10902 * t29102;
    let t29183 = t29182 * t10906;
    let t29188 = 0.35089340384731224426e1_f64 * t7151 * t8748 - 0.51947267698127589897e2_f64 * t1735 * t29074 - 0.62182e-1_f64 * t29099 * t621 - 0.35089340384731224426e1_f64 * t1735 * t29104 + 0.35089340384731224426e1_f64 * t1735 * t29108 + 0.1038945353962551798e3_f64 * t1735 * t29112 - 0.58482233974552040708e0_f64 * t1735 * t29175 - 0.17544670192365612213e1_f64 * t7151 * t8765 - 0.51947267698127589899e2_f64 * t7151 * t8769 - 0.1025389702100779493e4_f64 * t1735 * t29183 - 0.17544670192365612213e1_f64 * t23709 * t2432;
    t29188
}
