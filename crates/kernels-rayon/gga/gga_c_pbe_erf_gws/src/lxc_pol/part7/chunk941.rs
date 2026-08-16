//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 941/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk941(t17512: f64, t639: f64, t1892: f64, t5463: f64, t1620: f64, t5493: f64, t5505: f64, t5489: f64, t4913: f64, t5494: f64, t17490: f64, t17493: f64, t17498: f64, t17501: f64, t17503: f64, t17507: f64, t17511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17513 = t639 * t17512;
    let t17514 = 128.0_f64 / 1215.0_f64 * t17513;
    let t17516 = t639 * t5463 * t1892;
    let t17517 = 32.0_f64 / 135.0_f64 * t17516;
    let t17519 = t1620 * t5493 * t5505;
    let t17520 = 32.0_f64 / 15.0_f64 * t17519;
    let t17522 = t639 * t5493 * t5489;
    let t17523 = 32.0_f64 / 15.0_f64 * t17522;
    let t17524 = t4913 * t5494;
    let t17525 = 64.0_f64 / 15.0_f64 * t17524;
    let t17526 = 8.0_f64 * t17490 - t17493 - t17498 - t17501 + t17503 + t17507 + t17511 + t17514 + t17517 - t17520 + t17523 - t17525;
    (t17514, t17517, t17520, t17523, t17525, t17526)
}
