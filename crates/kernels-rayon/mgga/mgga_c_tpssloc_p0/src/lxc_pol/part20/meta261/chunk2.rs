//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1403/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1403(t10224: f64, t979: f64, t973: f64, t2970: f64, t2999: f64, t135: f64, t2978: f64) -> (f64, f64, f64, f64, f64) {
    let t10225 = t10224 * t979;
    let t10226 = t973 * t10225;
    let t10228 = t2970 * t2999;
    let t10229 = t973 * t10228;
    let t10231 = t135 * t2978;
    (t10225, t10226, t10228, t10229, t10231)
}
