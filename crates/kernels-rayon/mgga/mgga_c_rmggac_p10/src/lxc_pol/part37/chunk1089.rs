//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1089/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1089(t75806: f64, t75808: f64, t75828: f64, t75831: f64, t75834: f64, t75838: f64, t75841: f64, t75844: f64, t75847: f64, t75850: f64, t78304: f64, t78308: f64, t78309: f64, t78310: f64, t78311: f64, t78312: f64, t78313: f64) -> f64 {
    let t80333 = -t78304 - 0.87596530464506835932e-6_f64 * t75806 + 0.87596530464506835932e-6_f64 * t75808 + t78308 + t78309 + t78310 + t78311 + t78312 - t78313 - 0.10511583655740820312e-5_f64 * t75828 + 0.15767375483611230468e-5_f64 * t75831 - 0.21023167311481640624e-5_f64 * t75834 - t75838 + t75841 + t75844 - t75847 + t75850;
    t80333
}
