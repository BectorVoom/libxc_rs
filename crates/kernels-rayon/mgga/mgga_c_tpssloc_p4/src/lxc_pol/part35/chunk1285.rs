//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1285/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1285(t23143: f64, t7525: f64, t25316: f64, t82038: f64, t23171: f64, t23228: f64, t7488: f64, t23030: f64, t25205: f64, t1519: f64, t212: f64, t6554: f64) -> (f64, f64, f64, f64, f64) {
    let t87666 = t23143 * t7525;
    let t87718 = t82038 * t25316;
    let t87779 = t23171 * t23228 * t7488;
    let t87898 = t23030 * t25205;
    let t87915 = t23171 * t212 * t1519 * t6554;
    (t87666, t87718, t87779, t87898, t87915)
}
