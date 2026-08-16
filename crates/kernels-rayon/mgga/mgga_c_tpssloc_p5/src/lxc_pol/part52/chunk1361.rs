//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1361/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1361(t26135: f64, t7010: f64, t1873: f64, t86656: f64, t12524: f64, t33193: f64, t20173: f64, t33188: f64, t3941: f64, t6534: f64, t7467: f64, t26523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120812 = t7010 * t26135;
    let t120815 = t86656 * t1873;
    let t120818 = 27.0_f64 * t12524 * t33193;
    let t120820 = 54.0_f64 * t20173 * t33188;
    let t120823 = 54.0_f64 * t3941 * t6534 * t7467;
    let t120826 = t26523 * t6534;
    (t120812, t120815, t120818, t120820, t120823, t120826)
}
