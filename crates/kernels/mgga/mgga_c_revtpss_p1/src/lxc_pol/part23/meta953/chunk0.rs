//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3163/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3163<F: Float>(t1208: F, t24697: F, t225: F, t480: F, t17438: F, t20846: F, t5326: F, t6594: F, t1238: F, t12787: F, t17183: F, t17736: F, t17934: F, t21013: F, t21046: F, t24729: F, t3626: F, t3720: F, t5230: F, t5297: F, t5335: F, t5340: F, t5343: F, t6421: F, t70064: F, t70076: F, t70311: F, t70530: F, t71029: F) -> (F, F, F) {
    let t83107 = t24697 * t1208;
    let t83108 = t83107 * t225;
    let t83109 = t83108 * t480;
    let t83112 = t17438 * t20846;
    let t83114 = t5326 * t6594;
    let t83117 = F::cast_from(0.47637797908966374413e-3_f64) * t70064 + F::cast_from(0.14291339372689912324e-2_f64) * t17736 * t12787 * t6421 * t5230 + F::cast_from(0.64311027177104605458e-3_f64) * t70530 * t21046 - F::cast_from(0.13719685797782315831e-1_f64) * t17934 * t21013 * t5343 + F::cast_from(0.68598428988911579154e-2_f64) * t17183 * t21013 * t5335 - F::cast_from(0.85748036236139473944e-3_f64) * t17736 * t3626 * t71029 * t5297 + F::cast_from(0.12862205435420921092e-2_f64) * t5340 * t3720 * t70311 * t24729 - F::cast_from(0.57165357490759649296e-3_f64) * t70076 - F::cast_from(0.21437009059034868486e-3_f64) * t83109 * t1238 - F::cast_from(0.45732285992607719436e-2_f64) * t83112 - F::cast_from(0.21722835846488666732e-1_f64) * t83114 * t1238;
    (t83107, t83108, t83117)
}
