//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1006/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1006<F: Float>(t11422: F, t11423: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F, t23536: F, t23538: F, t23541: F, t23543: F) -> F {
    let t23753 = F::cast_from(0.34431666666666666666e0_f64) * t18919 - F::cast_from(0.103295e1_f64) * t18924 + F::cast_from(0.51647499999999999999e0_f64) * t18934 - t11422 - t11423 + F::cast_from(0.69463333333333333335e-1_f64) * t19002 - F::cast_from(0.41678000000000000001e0_f64) * t19004 + F::cast_from(0.20839e0_f64) * t19009 - F::cast_from(0.52945875e1_f64) * t23521 + F::cast_from(0.94674375e0_f64) * t23523 + F::cast_from(0.3529725e1_f64) * t23536 + F::cast_from(0.6311625e0_f64) * t23538 + F::cast_from(0.264729375e1_f64) * t23541 - F::cast_from(0.157790625e0_f64) * t23543;
    t23753
}
