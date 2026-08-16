//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2171/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2171(t3374: f64, t3399: f64, t440: f64, t3256: f64, t3263: f64, t1094: f64, t11189: f64, t1124: f64, t11349: f64, t3355: f64, t427: f64, t3358: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44154 = 1.0_f64 / t3399 / t3374;
    let t44155 = t440 * t44154;
    let t44159 = t3256 * t3263;
    let t44162 = t1094 * t11189;
    let t44172 = t1124 * t11349;
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    (t44154, t44155, t44159, t44162, t44172, t44177, t44178)
}
