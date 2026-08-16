//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2066/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2066(t3809: f64, t40281: f64, t12267: f64, t3865: f64, t1369: f64, t1362: f64, t40118: f64, t12344: f64, t3777: f64, t12361: f64, t3866: f64, t12331: f64, t1358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40282 = t40281 * t3809;
    let t40284 = t12267 * t3865;
    let t40285 = t40284 * t1369;
    let t40287 = t40118 * t1362;
    let t40292 = t3777 * t12344;
    let t40293 = t40292 * t1369;
    let t40295 = t3866 * t12361;
    let t40329 = t12331 * t1358;
    (t40282, t40284, t40285, t40287, t40292, t40293, t40295, t40329)
}
