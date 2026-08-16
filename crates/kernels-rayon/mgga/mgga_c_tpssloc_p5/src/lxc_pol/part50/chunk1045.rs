//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1045/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1045(t1945: f64, t6703: f64, t6706: f64, t8376: f64, t986: f64, t1921: f64, t30781: f64, t6705: f64, t6815: f64, t6704: f64, t8400: f64, t968: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30854 = t6703 * t1945;
    let t30855 = t30854 * t6706;
    let t30858 = t986 * t8376;
    let t30861 = t1921 * t30781;
    let t30862 = t986 * t30861;
    let t30868 = t6705 * t6815;
    let t30869 = t6704 * t30868;
    let t30874 = t968 * t8400;
    (t30854, t30855, t30858, t30861, t30862, t30868, t30869, t30874)
}
