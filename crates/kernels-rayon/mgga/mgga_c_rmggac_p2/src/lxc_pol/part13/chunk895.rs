//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 895/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk895(t39832: f64, t7478: f64, t1175: f64, t1971: f64, t515: f64, t570: f64, t8517: f64, t34884: f64, t9046: f64, t2289: f64, t34881: f64, t16501: f64, t7363: f64) -> (f64, f64, f64, f64, f64) {
    let t39833 = t39832 * t7478;
    let t39838 = t8517 * t1971 * t515 * t570 * t1175;
    let t39840 = t34884 * t9046;
    let t39842 = t34881 * t2289;
    let t39850 = t7363 * t16501;
    (t39833, t39838, t39840, t39842, t39850)
}
