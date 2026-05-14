//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1166/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1166<F: Float>(t3073: F, t6143: F, t6199: F, t1184: F, t6142: F, t6331: F, t237: F, t8040: F, t900: F, t3153: F, t6117: F, t2328: F, t8296: F, t2321: F, t3135: F, t6121: F, t898: F) -> (F, F, F, F, F, F) {
    let t22390 = 0.57895126195293126241e3 * t6199 * t3073 * t6143;
    let t22391 = t6142 * t1184;
    let t22393 = 0.2894756309764656312e3 * t22391 * t6331;
    let t22394 = t237 * t8040;
    let t22396 = 0.17544670867903938621e1 * t22394 * t900;
    let t22398 = 0.35089341735807877242e1 * t6117 * t3153;
    let t22400 = 0.35089341735807877242e1 * t2328 * t8296;
    let t22404 = 0.31168546390226634765e3 * t898 * t6121 * t3135 * t2321;
    (t22390, t22393, t22396, t22398, t22400, t22404)
}
