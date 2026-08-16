//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 913/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk913(t16043: f64, t9096: f64, t1971: f64, t2144: f64, t27044: f64, t3351: f64, t2604: f64, t35327: f64, t35337: f64, t39715: f64, t39717: f64, t39721: f64, t39726: f64, t39731: f64, t39733: f64, t39735: f64, t39742: f64, t39748: f64, t39752: f64, t39754: f64, t39756: f64, t39758: f64, t8378: f64) -> f64 {
    let t39760 = t16043 * t9096;
    let t39764 = t3351 * t1971 * t2144 * t27044;
    let t39766 = 0.12769379967989351819e-4_f64 * t39715 - 0.12769379967989351819e-4_f64 * t39717 - 0.25538759935978703638e-4_f64 * t39721 + 0.12769379967989351819e-4_f64 * t39726 + 0.42564599893297839398e-5_f64 * t39731 - 0.42564599893297839398e-5_f64 * t39733 + 0.85129199786595678796e-5_f64 * t39735 + 0.23948483403727617128e0_f64 * t2604 * t8378 - 0.12769379967989351819e-3_f64 * t39742 - 0.66211599834018861286e-4_f64 * t35327 - 0.59590439850616975158e-4_f64 * t35337 + 0.51077519871957407276e-4_f64 * t39748 - 0.76616279807936110914e-4_f64 * t39752 - 0.85129199786595678796e-5_f64 * t39754 - 0.53205749866622299248e-5_f64 * t39756 - 0.1064114997332445985e-4_f64 * t39758 + 0.25538759935978703638e-4_f64 * t39760 + 0.25538759935978703638e-4_f64 * t39764;
    t39766
}
