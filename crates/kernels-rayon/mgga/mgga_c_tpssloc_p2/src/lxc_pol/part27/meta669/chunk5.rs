//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2368/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2368(t90030: f64, t90422: f64, t91574: f64, t91617: f64, t91663: f64, t91709: f64, t91750: f64, t91789: f64, t26135: f64, t3941: f64, t671: f64, t2363: f64, t7467: f64) -> (f64, f64, f64) {
    let t91792 = t90030 + t90422 + t91574 + t91617 + t91663 + t91709 + t91750 + t91789;
    let t91799 = 54.0_f64 * t3941 * t26135 * t671;
    let t91802 = 27.0_f64 * t3941 * t7467 * t2363;
    (t91792, t91799, t91802)
}
