//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1161/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1161(t23165: f64, t82038: f64, t10046: f64, t1880: f64, t1894: f64, t214: f64, t1879: f64, t80845: f64, t1906: f64, t23035: f64, t23153: f64, t2379: f64, t6637: f64) -> (f64, f64, f64, f64, f64) {
    let t82039 = t82038 * t23165;
    let t82043 = t1880 * t214 * t1894 * t10046;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    let t82050 = t23035 * t6637 * t23153 * t2379;
    (t82039, t82043, t82045, t82046, t82050)
}
