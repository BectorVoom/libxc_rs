//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2042/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2042(t1369: f64, t40059: f64, t22843: f64, t241: f64, t67: f64, t10021: f64, t1336: f64, t1339: f64, t1354: f64, t12384: f64, t3777: f64, t12282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40060 = t40059 * t1369;
    let t40070 = t241 * t22843 * t67;
    let t40123 = t1336 * t1339 * t10021;
    let t40124 = t40123 * t1354;
    let t40130 = t3777 * t12384;
    let t40138 = t3777 * t12282;
    (t40060, t40070, t40123, t40124, t40130, t40138)
}
