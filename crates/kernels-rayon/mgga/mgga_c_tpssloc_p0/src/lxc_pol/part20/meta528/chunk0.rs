//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2062/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2062(t12300: f64, t3853: f64, t12305: f64, t3866: f64, t12238: f64, t68: f64, t1340: f64, t10021: f64, t1336: f64, t1339: f64, t1354: f64, t12365: f64, t3858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40114 = t12300 * t3853;
    let t40116 = t3866 * t12305;
    let t40118 = t12238 * t68;
    let t40119 = t40118 * t1340;
    let t40123 = t1336 * t1339 * t10021;
    let t40124 = t40123 * t1354;
    let t40126 = t12365 * t3858;
    (t40114, t40116, t40118, t40119, t40123, t40124, t40126)
}
