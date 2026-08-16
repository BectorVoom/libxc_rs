//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2650/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2650(t12189: f64, t5227: f64, t16232: f64, t3777: f64, t40281: f64, t5303: f64, t12211: f64, t16300: f64, t5247: f64, t820: f64, t12250: f64, t1824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53984 = t12189 * t5227;
    let t53985 = 35.0_f64 / 72.0_f64 * t53984;
    let t53990 = t3777 * t16232;
    let t53997 = t40281 * t5303;
    let t53998 = 119.0_f64 / 1152.0_f64 * t53997;
    let t54003 = t12211 * t16300;
    let t54013 = t5247 * t820;
    let t54014 = t1824 * t12250;
    (t53985, t53990, t53998, t54003, t54013, t54014)
}
