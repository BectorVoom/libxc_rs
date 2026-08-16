//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 784/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk784(t12709: f64, t198: f64, t186: f64, t561: f64, t1019: f64, t3399: f64, t2790: f64, t3451: f64, t10326: f64, t1006: f64, t3445: f64, t12705: f64, t12707: f64, t12713: f64, t12715: f64, t12719: f64, t12721: f64, t12725: f64, t12726: f64, t12728: f64, t5933: f64, t5944: f64, t8440: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12729 = -t12709;
    let t12730 = t198 * t12729;
    let t12731 = t186 * t12730;
    let t12733 = 4.0_f64 / 15.0_f64 * t561 * t12731;
    let t12735 = 4.0_f64 / 5.0_f64 * t3399 * t1019;
    let t12737 = 4.0_f64 / 5.0_f64 * t2790 * t3451;
    let t12739 = 4.0_f64 / 5.0_f64 * t10326 * t3451;
    let t12741 = 2.0_f64 / 5.0_f64 * t1006 * t3445;
    let t12742 = t5933 - t5944 + 8.0_f64 * t8440 - t12705 + t12707 + t12713 + t12715 - t12719 + t12721 + t12725 - t12726 - t12728 + t12733 - t12735 + t12737 + t12739 - t12741;
    (t12729, t12730, t12731, t12733, t12735, t12737, t12739, t12741, t12742)
}
