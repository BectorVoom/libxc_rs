//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2110/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2110(t112: f64, t26509: f64, t16535: f64, t7467: f64, t26135: f64, t3938: f64, t12816: f64, t191: f64, t192: f64, t2020: f64, t26161: f64, t26162: f64, t56404: f64) -> (f64, f64, f64, f64, f64) {
    let t86656 = t26509 * t112;
    let t86660 = 27.0_f64 * t16535 * t7467;
    let t86668 = 27.0_f64 * t3938 * t26135;
    let t86672 = t12816 * t191 * t192;
    let t86673 = t86672 * t2020;
    let t86676 = 4.0_f64 * t26161 * t26162 * t56404;
    (t86656, t86660, t86668, t86673, t86676)
}
