//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1109/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1109(t118: f64, t326: f64, t35865: f64, t35869: f64, t35873: f64, t35877: f64, t35886: f64, t35890: f64, t37439: f64, t41077: f64, t41079: f64, t41084: f64, t43163: f64, t43975: f64) -> f64 {
    let t44130 = -t37439 - 0.72732431077987577948e-1_f64 * t35865 - 0.18183107769496894487e-1_f64 * t35869 + 0.54549323308490683461e-1_f64 * t35873 - 0.40002837092893167872e0_f64 * t35877 + 0.36366215538993788974e0_f64 * t35886 + 0.10909864661698136692e0_f64 * t35890 - 0.11974241701863808564e0_f64 * t326 * t43163 + 0.11974241701863808564e0_f64 * t118 * t43975 - 0.17961362552795712846e1_f64 * t41077 + 0.11974241701863808564e0_f64 * t41079 - 0.35922725105591425692e0_f64 * t41084;
    t44130
}
