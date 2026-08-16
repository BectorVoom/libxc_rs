//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1063/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1063(t39873: f64, t39899: f64, t35473: f64, t35478: f64, t35481: f64, t35484: f64, t35487: f64, t35514: f64, t35516: f64, t37720: f64, t37731: f64, t39871: f64, t39877: f64, t39881: f64, t39889: f64, t39893: f64, t39901: f64, t39907: f64) -> f64 {
    let t43169 = 0.39726959900411316772e-4_f64 * t39873;
    let t43179 = 0.10909864661698136692e0_f64 * t39899;
    let t43184 = 0.5107751987195740728e-4_f64 * t39871 + t43169 + 0.40911992481368012596e-1_f64 * t39877 - 0.81823984962736025192e-1_f64 * t39881 + t37720 + 0.19863479950205658386e-4_f64 * t35473 + 0.85129199786595678799e-5_f64 * t39889 + 0.23942587439980034662e-4_f64 * t39893 + 0.325201597776800302e-2_f64 * t35478 - 0.78064147182743091556e-3_f64 * t35481 + 0.325201597776800302e-2_f64 * t35484 - 0.78064147182743091556e-3_f64 * t35487 + t37731 - t43179 + 0.19863479950205658387e-3_f64 * t39901 + 0.13242319966803772257e-3_f64 * t35514 + 0.39726959900411316772e-4_f64 * t35516 - 0.20431007948782962912e-3_f64 * t39907;
    t43184
}
