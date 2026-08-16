//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2683/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2683(t54643: f64, t40343: f64, t40347: f64, t40350: f64, t40351: f64, t40356: f64, t54631: f64, t54633: f64, t54635: f64, t54638: f64, t54639: f64, t40360: f64, t40366: f64, t40372: f64, t40376: f64, t40387: f64, t40401: f64, t40402: f64, t40404: f64, t40407: f64, t40410: f64, t40415: f64, t40422: f64) -> (f64, f64) {
    let t54644 = 0.14999999999999999999e-1_f64 * t54643;
    let t54647 = -t40343 + t40347 + t40350 - 0.38888888888888888887e-1_f64 * t54631 + 0.32870370370370370369e-1_f64 * t54633 + 0.11666666666666666666e-1_f64 * t54635 - t54638 + 0.56172839506172839502e-1_f64 * t54639 - t54644 - 0.59999999999999999997e-1_f64 * t40351 - 0.15e-1_f64 * t40356;
    let t54658 = 0.49999999999999999998e-2_f64 * t40360 - 0.34999999999999999998e-1_f64 * t40366 + 0.83333333333333333331e-3_f64 * t40372 - 0.75e-2_f64 * t40376 + 0.11666666666666666666e0_f64 * t40387 - t40401 + 0.16851851851851851851e0_f64 * t40402 - 0.38888888888888888889e-1_f64 * t40404 + 0.98611111111111111108e-1_f64 * t40407 + 0.47499999999999999998e-1_f64 * t40410 + 0.1e-1_f64 * t40415 + t40422;
    (t54647, t54658)
}
