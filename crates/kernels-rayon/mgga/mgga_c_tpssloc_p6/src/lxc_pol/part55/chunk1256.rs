//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1256/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1256(t12524: f64, t33193: f64, t20173: f64, t33188: f64, t3941: f64, t6534: f64, t7467: f64, t1873: f64, t26135: f64, t4072: f64, t576: f64, t8319: f64) -> (f64, f64, f64, f64, f64) {
    let t120818 = 27.0_f64 * t12524 * t33193;
    let t120820 = 54.0_f64 * t20173 * t33188;
    let t120823 = 54.0_f64 * t3941 * t6534 * t7467;
    let t120830 = 54.0_f64 * t3941 * t1873 * t26135;
    let t120833 = t576 * t4072;
    let t120835 = 27.0_f64 * t120833 * t8319;
    (t120818, t120820, t120823, t120830, t120835)
}
