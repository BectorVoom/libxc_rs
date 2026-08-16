//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1196/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1196(t25305: f64, t92868: f64, t25292: f64, t25322: f64, t25326: f64, t25344: f64, t25383: f64, t25391: f64, t25394: f64, t2772: f64, t92841: f64, t92844: f64, t92847: f64, t92856: f64, t92858: f64, t92861: f64, t92864: f64, t92870: f64, t92873: f64) -> f64 {
    let t92875 = 0.91399340044406952588e-2_f64 * t25305 * t92868;
    let t92876 = -0.15421710918628844643e0_f64 * t92841 + 0.86736281882051994623e-1_f64 * t92844 + 0.29272321618148349057e-1_f64 * t92847 + 0.26020884564615598386e1_f64 * t25383 * t25344 + 0.26020884564615598386e1_f64 * t25383 * t25326 + 0.52041769129231196772e1_f64 * t25383 * t25292 + 0.16463622957338778996e-1_f64 * t92856 - 0.21951497276451705329e-1_f64 * t92858 + t92861 + 0.39512695097613069591e1_f64 * t25322 * t2772 - 0.52041769129231196772e1_f64 * t25391 * t92864 * t25394 - t92870 - t92873 + t92875;
    t92876
}
