//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2548/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2548(t423: f64, t51570: f64, t51590: f64, t1128: f64, t15204: f64, t3356: f64, t4794: f64, t11349: f64, t1675: f64, t14829: f64, t3403: f64, t11297: f64, t11345: f64, t11353: f64, t1138: f64, t11434: f64, t1155: f64, t15126: f64, t15141: f64, t15179: f64, t15182: f64, t15185: f64, t1683: f64, t3352: f64, t3360: f64, t3401: f64, t44202: f64, t44205: f64, t44295: f64, t44300: f64, t4797: f64, t4824: f64, t4840: f64, t51549: f64) -> (f64, f64) {
    let t51593 = 0.621814e-1_f64 * (t51570 + t51590) * t423;
    let t51594 = t15204 * t1128;
    let t51599 = t4794 * t3356;
    let t51604 = t1675 * t11349;
    let t51613 = t14829 * t3403;
    let t51617 = -0.35089341735807877242e1_f64 * t44202 * t4840 - 0.70178683471615754484e1_f64 * t11297 * t15179 - 0.35089341735807877242e1_f64 * t11297 * t15182 - 0.31168546390226634765e3_f64 * t44205 * t15185 + t51549 + t51593 + 3.0_f64 * t51594 * t1138 + 3.0_f64 * t15141 * t3352 + 0.96491876992155210402e2_f64 * t51599 * t3360 + 1.0_f64 * t4797 * t11345 + 0.2069040516770936012e4_f64 * t51604 * t11353 + 1.0_f64 * t44295 * t1683 + 0.96491876992155210402e2_f64 * t44300 * t4824 + 0.51947577317044391277e2_f64 * t15126 * t11434 + 0.51947577317044391277e2_f64 * t3401 * t51613 * t1155;
    (t51593, t51617)
}
