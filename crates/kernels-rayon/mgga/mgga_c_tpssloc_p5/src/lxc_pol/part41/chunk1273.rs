//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1273/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1273(t112: f64, t8199: f64, t111: f64, t2205: f64, t2585: f64, t656: f64, t1849: f64, t8189: f64, t2199: f64, t5361: f64, t1266: f64, t8273: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30109 = t8199 * t112;
    let t30112 = t2205 * t111;
    let t30175 = t2585 * t656;
    let t30266 = t8189 * t1849;
    let t30269 = t2199 * t5361;
    let t30272 = t1266 * t8273;
    (t30109, t30112, t30175, t30266, t30269, t30272)
}
