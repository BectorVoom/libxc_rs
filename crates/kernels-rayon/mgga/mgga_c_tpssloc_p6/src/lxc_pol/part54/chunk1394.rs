//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1394/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1394(t112936: f64, t112942: f64, t114916: f64, t114933: f64, t114939: f64, t13042: f64, t1492: f64, t24305: f64, t25348: f64, t259: f64, t26700: f64, t31343: f64, t31361: f64, t31423: f64, t4142: f64, t4147: f64, t4273: f64, t6632: f64, t7092: f64, t7538: f64, t8543: f64, t8553: f64) -> f64 {
    let t121711 = 0.82246703342411321824e-2_f64 * t114916 + t4142 * t8543 * t259 + t1492 * t31361 * t259 + 2.0_f64 * t13042 * t8553 + 2.0_f64 * t25348 * t7092 + 2.0_f64 * t4147 * t31343 + t112936 + 2.0_f64 * t26700 * t6632 - t24305 * t7538 + 2.0_f64 * t31423 * t4273 - t114933 - t112942 + 0.19190897446562641759e-1_f64 * t114939;
    t121711
}
