//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 923/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk923(t2318: f64, t3369: f64, t34975: f64, t7482: f64, t2131: f64, t5026: f64, t35478: f64, t35481: f64, t35484: f64, t35487: f64, t35497: f64, t35514: f64, t35516: f64, t39889: f64, t39893: f64, t39899: f64, t39901: f64, t39907: f64, t39911: f64, t39915: f64, t39917: f64) -> f64 {
    let t39921 = t34975 * t3369 * t2318 * t7482;
    let t39923 = t5026 * t2131;
    let t39925 = 0.42564599893297839398e-5_f64 * t39889 + 0.11971293719990017331e-4_f64 * t39893 + 0.16260079888840015101e-2_f64 * t35478 - 0.3903207359137154578e-3_f64 * t35481 + 0.16260079888840015101e-2_f64 * t35484 - 0.3903207359137154578e-3_f64 * t35487 + t35497 - 0.54549323308490683457e-1_f64 * t39899 + 0.99317399751028291927e-4_f64 * t39901 + 0.66211599834018861286e-4_f64 * t35514 + 0.19863479950205658386e-4_f64 * t35516 - 0.10215503974391481455e-3_f64 * t39907 + 0.31923449919973379548e-4_f64 * t39911 - 0.25538759935978703638e-4_f64 * t39915 + 0.25538759935978703638e-4_f64 * t39917 - 0.31923449919973379548e-4_f64 * t39921 - 0.2363e1_f64 * t39923;
    t39925
}
