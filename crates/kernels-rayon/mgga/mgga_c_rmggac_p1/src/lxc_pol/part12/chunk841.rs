//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 841/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk841(t29892: f64, t3351: f64, t3352: f64, t515: f64, t2010: f64, t2012: f64, t5061: f64, t34705: f64, t34707: f64, t34711: f64, t34713: f64, t34717: f64, t38695: f64, t38699: f64, t38702: f64, t38705: f64, t38708: f64, t38710: f64, t38712: f64, t38717: f64, t38719: f64, t38724: f64, t4985: f64, t7564: f64) -> f64 {
    let t38728 = t3351 * t3352 * t515 * t29892;
    let t38733 = t2010 * t2012 * t5061;
    let t38735 = -0.8980681276397856423e-1_f64 * t38695 + t34705 + t34707 - t34711 - 0.51240438831339423711e-4_f64 * t34713 + 0.72042316457491791906e-3_f64 * t34717 - 0.85129199786595678796e-5_f64 * t38699 + 0.85129199786595678796e-5_f64 * t38702 + t38705 - 0.76616279807936110914e-4_f64 * t38708 - 0.23836175940246790062e-3_f64 * t38710 - 0.59590439850616975156e-4_f64 * t38712 - 0.25538759935978703638e-4_f64 * t38717 - 0.25538759935978703638e-4_f64 * t38719 + 0.85129199786595678796e-5_f64 * t38724 - 0.25538759935978703638e-4_f64 * t38728 + 0.11974241701863808564e0_f64 * t4985 * t7564 - 0.72042316457491791906e-3_f64 * t38733;
    t38735
}
