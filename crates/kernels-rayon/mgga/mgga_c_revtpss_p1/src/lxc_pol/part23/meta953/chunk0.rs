//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3163/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3163(t1208: f64, t24697: f64, t225: f64, t480: f64, t17438: f64, t20846: f64, t5326: f64, t6594: f64, t1238: f64, t12787: f64, t17183: f64, t17736: f64, t17934: f64, t21013: f64, t21046: f64, t24729: f64, t3626: f64, t3720: f64, t5230: f64, t5297: f64, t5335: f64, t5340: f64, t5343: f64, t6421: f64, t70064: f64, t70076: f64, t70311: f64, t70530: f64, t71029: f64) -> (f64, f64, f64) {
    let t83107 = t24697 * t1208;
    let t83108 = t83107 * t225;
    let t83109 = t83108 * t480;
    let t83112 = t17438 * t20846;
    let t83114 = t5326 * t6594;
    let t83117 = 0.47637797908966374413e-3_f64 * t70064 + 0.14291339372689912324e-2_f64 * t17736 * t12787 * t6421 * t5230 + 0.64311027177104605458e-3_f64 * t70530 * t21046 - 0.13719685797782315831e-1_f64 * t17934 * t21013 * t5343 + 0.68598428988911579154e-2_f64 * t17183 * t21013 * t5335 - 0.85748036236139473944e-3_f64 * t17736 * t3626 * t71029 * t5297 + 0.12862205435420921092e-2_f64 * t5340 * t3720 * t70311 * t24729 - 0.57165357490759649296e-3_f64 * t70076 - 0.21437009059034868486e-3_f64 * t83109 * t1238 - 0.45732285992607719436e-2_f64 * t83112 - 0.21722835846488666732e-1_f64 * t83114 * t1238;
    (t83107, t83108, t83117)
}
