//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1153/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1153<F: Float>(t2215: F, t7958: F, t836: F, t2209: F, t7966: F, t3041: F, t6158: F, t7972: F, t3052: F, t7996: F, t7999: F, t18427: F, t18430: F, t18433: F, t18440: F, t18443: F, t18445: F, t18448: F, t22190: F, t22193: F, t22196: F, t22199: F, t22202: F) -> (F, F, F, F, F, F, F, F) {
    let t22205 = t2215 * t7958 * t836;
    let t22207 = t7966 * t2209;
    let t22209 = t3041 * t6158;
    let t22215 = t7972 * t2209;
    let t22217 = t3052 * t6158;
    let t22219 = t2209 * t836;
    let t22220 = t7996 * t22219;
    let t22222 = t7999 * t22219;
    let t22225 = 0.58258125e1 * t22190 - 0.1237865625e0 * t22193 - 0.485484375e1 * t22196 + 0.6189328125e-1 * t22199 - 0.3883875e1 * t22202 + 0.247573125e0 * t22205 - 0.3883875e1 * t22207 - 0.1294625e1 * t22209 + t18440 - 0.28179666666666666667e1 * t18427 + 0.12077e1 * t18430 - 0.301925e0 * t18433 + t18443 + 0.82785e0 * t18448 + 0.247573125e0 * t22215 + 0.82524375e-1 * t22217 + 0.58258125e1 * t22220 - 0.1237865625e0 * t22222 - 0.22076e1 * t18445;
    (t22205, t22207, t22209, t22215, t22217, t22220, t22222, t22225)
}
