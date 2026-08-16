//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1157/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1157(t193: f64, t1962: f64, t10143: f64, t25: f64, t28: f64, t870: f64, t1437: f64, t1864: f64, t1410: f64, t2240: f64, t1453: f64, t22470: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25372 = t193 * t1962;
    let t25373 = t10143 * t25;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26012 = t1864 * t1437;
    let t26016 = t2240 * t1410;
    let t26127 = t22470 * t1453;
    (t25372, t25373, t25891, t25927, t26012, t26016, t26127)
}
