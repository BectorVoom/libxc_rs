//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2024/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2024<F: Float>(t2453: F, t25949: F, t25946: F, t25939: F, t40270: F, t10073: F, t25920: F, t25938: F, t25898: F, t94889: F, t10115: F, t2024: F) -> (F, F, F, F, F) {
    let t94913 = t2453 * t25949;
    let t94914 = t94913 * t25946;
    let t94917 = F::cast_from(0.96373646535613327356e-3_f64) * t40270 * t25939;
    let t94919 = t10073 * t25920 * t25938;
    let t94921 = t94889 * t25898;
    let t94931 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t2024;
    (t94914, t94917, t94919, t94921, t94931)
}
