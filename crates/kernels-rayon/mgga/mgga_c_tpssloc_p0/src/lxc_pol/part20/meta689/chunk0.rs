//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2611/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2611(t11678: f64, t11697: f64, t15559: f64, t15713: f64, t3577: f64, t45124: f64, t1213: f64, t1735: f64, t248: f64, t45017: f64, t10477: f64, t1742: f64) -> (f64, f64, f64, f64) {
    let t53064 = t11678 * t11697 * t15559;
    let t53067 = t3577 * t45124 * t15713;
    let t53079 = t1213 * t248 * t45017 * t1735;
    let t53081 = t1742 * t10477;
    (t53064, t53067, t53079, t53081)
}
