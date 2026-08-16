//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1345/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1345(t3008: f64, t10199: f64, t2970: f64, t973: f64, t10200: f64, t10214: f64, t10219: f64, t10235: f64, t10278: f64, t2960: f64, t2986: f64, t340: f64, t343: f64, t39097: f64, t39110: f64, t42968: f64, t42974: f64, t42976: f64, t42985: f64, t43000: f64, t43012: f64, t974: f64, t977: f64, t978: f64) -> f64 {
    let t43019 = t3008 * t3008;
    let t43028 = t973 * t2970 * t10199;
    let t43034 = 0.19753086419753086419e-2_f64 * t42968 - 0.92181069958847736624e-2_f64 * t2960 * t10219 + 0.11522633744855967078e-2_f64 * t42974 - 0.1037037037037037037e-1_f64 * t973 * t10214 * t42976 * t39097 + 0.27777777777777777777e-3_f64 * t973 * t977 * t978 * t39110 - 0.44444444444444444444e-2_f64 * t2986 * t10235 * t42985 - 0.83333333333333333332e-3_f64 * t973 * t974 * t340 * (t43000 + t43012) * t343 - 0.24999999999999999999e-2_f64 * t973 * t974 * t340 * t43019 * t343 - 0.17777777777777777777e-1_f64 * t2960 * t10200 + 0.22222222222222222221e-2_f64 * t43028 - 0.66666666666666666664e-2_f64 * t973 * t977 * t10278 * t39097;
    t43034
}
