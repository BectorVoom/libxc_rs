//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1277/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1277(t3073: f64, t6143: f64, t6199: f64, t1184: f64, t6142: f64, t6331: f64, t237: f64, t8040: f64, t900: f64, t3153: f64, t6117: f64, t2328: f64, t8296: f64) -> (f64, f64, f64, f64, f64) {
    let t22390 = 0.57895126195293126241e3_f64 * t6199 * t3073 * t6143;
    let t22391 = t6142 * t1184;
    let t22393 = 0.2894756309764656312e3_f64 * t22391 * t6331;
    let t22394 = t237 * t8040;
    let t22396 = 0.17544670867903938621e1_f64 * t22394 * t900;
    let t22398 = 0.35089341735807877242e1_f64 * t6117 * t3153;
    let t22400 = 0.35089341735807877242e1_f64 * t2328 * t8296;
    (t22390, t22393, t22396, t22398, t22400)
}
