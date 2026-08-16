//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1372/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1372(t708: f64, t9912: f64, t157: f64, t9448: f64, t182: f64, t2509: f64, t746: f64, t9490: f64) -> (f64, f64, f64, f64) {
    let t9914 = 12.0_f64 * t9912 * t708;
    let t9915 = t9448 * t157;
    let t9917 = 0.19751673498613801407e-1_f64 * t9915 * t182;
    let t9919 = t2509 * t9490 * t746;
    (t9914, t9915, t9917, t9919)
}
