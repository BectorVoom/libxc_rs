//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3146/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3146(t15294: f64, t15376: f64, t44573: f64, t44586: f64, t44635: f64, t44638: f64, t44641: f64, t52300: f64, t52354: f64, t52357: f64, t52362: f64, t52364: f64, t52367: f64) -> f64 {
    let t65161 = -0.82304526748971193413e-4_f64 * t44573 + 0.49382716049382716047e-3_f64 * t52300 - 0.12345679012345679012e-3_f64 * t44586 - 0.59259259259259259256e-2_f64 * t15376 * t15294 + 0.74074074074074074072e-3_f64 * t52354 - 0.18518518518518518518e-3_f64 * t52357 - 0.55555555555555555554e-3_f64 * t52362 - 0.65843621399176954729e-3_f64 * t52364 + 0.24691358024691358024e-3_f64 * t52367 - 0.20576131687242798354e-3_f64 * t44635 + 0.6172839506172839506e-4_f64 * t44638 + 0.12345679012345679012e-3_f64 * t44641;
    t65161
}
