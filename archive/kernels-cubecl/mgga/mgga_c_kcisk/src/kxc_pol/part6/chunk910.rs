//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 910/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk910<F: Float>(t29146: F, t29172: F, t1737: F, t1746: F, t10902: F, t29102: F, t10906: F, t1735: F, t23709: F, t2432: F, t29074: F, t29099: F, t29104: F, t29108: F, t29112: F, t621: F, t7151: F, t8748: F, t8765: F, t8769: F) -> F {
    let t29173 = t29146 + t29172;
    let t29175 = t1737 * t29173 * t1746;
    let t29182 = t10902 * t29102;
    let t29183 = t29182 * t10906;
    let t29188 = F::cast_from(0.35089340384731224426e1_f64) * t7151 * t8748 - F::cast_from(0.51947267698127589897e2_f64) * t1735 * t29074 - F::cast_from(0.62182e-1_f64) * t29099 * t621 - F::cast_from(0.35089340384731224426e1_f64) * t1735 * t29104 + F::cast_from(0.35089340384731224426e1_f64) * t1735 * t29108 + F::cast_from(0.1038945353962551798e3_f64) * t1735 * t29112 - F::cast_from(0.58482233974552040708e0_f64) * t1735 * t29175 - F::cast_from(0.17544670192365612213e1_f64) * t7151 * t8765 - F::cast_from(0.51947267698127589899e2_f64) * t7151 * t8769 - F::cast_from(0.1025389702100779493e4_f64) * t1735 * t29183 - F::cast_from(0.17544670192365612213e1_f64) * t23709 * t2432;
    t29188
}
