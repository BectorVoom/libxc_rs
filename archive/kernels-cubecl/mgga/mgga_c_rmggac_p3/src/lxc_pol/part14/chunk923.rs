//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 923/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk923<F: Float>(t2318: F, t3369: F, t34975: F, t7482: F, t2131: F, t5026: F, t35478: F, t35481: F, t35484: F, t35487: F, t35497: F, t35514: F, t35516: F, t39889: F, t39893: F, t39899: F, t39901: F, t39907: F, t39911: F, t39915: F, t39917: F) -> F {
    let t39921 = t34975 * t3369 * t2318 * t7482;
    let t39923 = t5026 * t2131;
    let t39925 = F::cast_from(0.42564599893297839398e-5_f64) * t39889 + F::cast_from(0.11971293719990017331e-4_f64) * t39893 + F::cast_from(0.16260079888840015101e-2_f64) * t35478 - F::cast_from(0.3903207359137154578e-3_f64) * t35481 + F::cast_from(0.16260079888840015101e-2_f64) * t35484 - F::cast_from(0.3903207359137154578e-3_f64) * t35487 + t35497 - F::cast_from(0.54549323308490683457e-1_f64) * t39899 + F::cast_from(0.99317399751028291927e-4_f64) * t39901 + F::cast_from(0.66211599834018861286e-4_f64) * t35514 + F::cast_from(0.19863479950205658386e-4_f64) * t35516 - F::cast_from(0.10215503974391481455e-3_f64) * t39907 + F::cast_from(0.31923449919973379548e-4_f64) * t39911 - F::cast_from(0.25538759935978703638e-4_f64) * t39915 + F::cast_from(0.25538759935978703638e-4_f64) * t39917 - F::cast_from(0.31923449919973379548e-4_f64) * t39921 - F::cast_from(0.2363e1_f64) * t39923;
    t39925
}
