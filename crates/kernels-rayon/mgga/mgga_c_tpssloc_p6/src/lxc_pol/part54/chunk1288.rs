//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1288/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1288(t114891: f64, t23168: f64, t31367: f64, t114790: f64, t23164: f64, t6555: f64, t2047: f64, t212: f64, t23171: f64, t6554: f64, t31420: f64, t6547: f64) -> (f64, f64, f64, f64, f64) {
    let t114892 = 0.26044789391763585244e-1_f64 * t114891;
    let t114900 = t23168 * t31367;
    let t114916 = t23164 * t114790 * t6555;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114933 = 0.82246703342411321824e-2_f64 * t114932;
    let t114939 = t6547 * t31420;
    (t114892, t114900, t114916, t114933, t114939)
}
