//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 881/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk881(t10024: f64, t34761: f64, t38415: f64, t38460: f64, t42600: f64, t44755: f64, t44759: f64, t44763: f64, t44767: f64, t44771: f64, t44773: f64, t44777: f64, t44781: f64, t44784: f64, t44786: f64, t44789: f64, t44793: f64, t44795: f64, t44799: f64) -> f64 {
    let t44801 = t34761 * t10024;
    let t44803 = -0.25538759935978703638e-4_f64 * t44755 + t42600 - 0.1064114997332445985e-4_f64 * t44759 - 0.25538759935978703638e-4_f64 * t44763 - 0.85129199786595678796e-5_f64 * t44767 + 0.25538759935978703638e-4_f64 * t44771 - 0.85129199786595678796e-5_f64 * t44773 - 0.23942587439980034662e-4_f64 * t44777 + t38415 - 0.31923449919973379548e-4_f64 * t44781 - 0.5586603735995341421e-4_f64 * t38460 + 0.59590439850616975155e-4_f64 * t44784 + 0.27274661654245341729e-1_f64 * t44786 + 0.13637330827122670864e-1_f64 * t44789 + 0.31923449919973379548e-4_f64 * t44793 + 0.20455996240684006296e-1_f64 * t44795 - 0.25538759935978703638e-4_f64 * t44799 + 0.25538759935978703638e-4_f64 * t44801;
    t44803
}
