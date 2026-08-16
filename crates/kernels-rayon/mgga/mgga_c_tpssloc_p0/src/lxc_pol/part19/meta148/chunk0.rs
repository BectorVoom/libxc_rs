//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 755/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk755(t2932: f64, t950: f64, t2978: f64, t60: f64, t344: f64, t2987: f64, t340: f64, t974: f64, t247: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4497 = t2932 * t950;
    let t4509 = t60 * t2978;
    let t4510 = t4509 * t344;
    let t4518 = t2987 * t344;
    let t4546 = t974 * t340;
    let t4582 = t247 * t375;
    (t4497, t4509, t4510, t4518, t4546, t4582)
}
