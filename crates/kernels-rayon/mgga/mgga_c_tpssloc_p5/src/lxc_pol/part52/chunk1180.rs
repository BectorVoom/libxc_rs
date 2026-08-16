//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1180/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1180(t2314: f64, t8326: f64, t5113: f64, t191: f64, t192: f64, t6872: f64, t2020: f64, t6876: f64, t8494: f64, t6997: f64, t8450: f64, t1873: f64, t23877: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31236 = t2314 * t8326;
    let t31237 = 2.0_f64 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0_f64 * t31238;
    let t31246 = t6872 * t191 * t192;
    let t31247 = t31246 * t2020;
    let t31249 = t6876 * t8494;
    let t31250 = t8450 * t6997;
    let t31270 = t23877 * t1873;
    (t31237, t31239, t31246, t31247, t31249, t31250, t31270)
}
