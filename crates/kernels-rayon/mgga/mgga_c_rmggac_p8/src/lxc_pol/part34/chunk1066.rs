//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1066/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1066(t75811: f64, t75814: f64, t75818: f64, t75820: f64, t75823: f64, t75825: f64, t75806: f64, t75808: f64, t75828: f64, t75831: f64, t75834: f64, t75838: f64, t75841: f64, t75844: f64, t75847: f64, t75850: f64, t75853: f64) -> f64 {
    let t78308 = 0.2627895913935205078e-5_f64 * t75811;
    let t78309 = 0.59127658063542114255e-5_f64 * t75814;
    let t78310 = 0.7661627980793611092e-4_f64 * t75818;
    let t78311 = 0.5959043985061697516e-4_f64 * t75820;
    let t78312 = 0.2553875993597870364e-4_f64 * t75823;
    let t78313 = 0.2553875993597870364e-4_f64 * t75825;
    let t78317 = -0.87596530464506835935e-6_f64 * t75806 + 0.87596530464506835935e-6_f64 * t75808 + t78308 + t78309 + t78310 + t78311 + t78312 - t78313 - 0.10511583655740820313e-5_f64 * t75828 + 0.15767375483611230469e-5_f64 * t75831 - 0.21023167311481640626e-5_f64 * t75834 - t75838 + t75841 + t75844 - t75847 + t75850 + t75853;
    t78317
}
