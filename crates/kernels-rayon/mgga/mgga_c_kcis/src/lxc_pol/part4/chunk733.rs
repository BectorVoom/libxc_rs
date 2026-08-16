//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 733/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk733(t4129: f64, t4293: f64, t4292: f64, t3954: f64, t584: f64, t583: f64, t1546: f64, t556: f64, t4136: f64, t578: f64, t3722: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4294 = t4293 * t4129;
    let t4295 = t4292 * t4294;
    let t4297 = t584 * t3954;
    let t4298 = t583 * t4297;
    let t4299 = t1546 * t4298;
    let t4301 = 1.0_f64 / t556;
    let t4302 = t4301 * t4136;
    let t4303 = t583 * t4302;
    let t4304 = t578 * t4303;
    let t4306 = t555 * t3722;
    (t4294, t4295, t4297, t4298, t4299, t4301, t4302, t4303, t4304, t4306)
}
