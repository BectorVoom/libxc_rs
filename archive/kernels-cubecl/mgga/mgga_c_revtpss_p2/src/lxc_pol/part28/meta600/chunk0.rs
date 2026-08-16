//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2077/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2077<F: Float>(t10073: F, t25920: F, t25938: F, t25898: F, t94889: F, t25901: F, t10115: F, t2024: F, t112: F, t843: F, t239: F, t655: F) -> (F, F, F, F, F, F) {
    let t94919 = t10073 * t25920 * t25938;
    let t94921 = t94889 * t25898;
    let t94922 = t94921 * t25901;
    let t94931 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t2024;
    let t94973 = t843 * t112;
    let t94974 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t94973;
    let t94975 = t239 * t655;
    (t94919, t94921, t94922, t94931, t94974, t94975)
}
