//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 808/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk808<F: Float>(t1652: F, t2615: F, t1009: F, t4991: F, t587: F, t2815: F, t586: F, t645: F, t7524: F, t7526: F, t7529: F, t7532: F, t7536: F, t7538: F, t7540: F, t7541: F, t7563: F, t7567: F, t7569: F, t7572: F, t7573: F, t7576: F) -> (F, F, F, F) {
    let t7578 = 16.0 / 135.0 * t2615 * t1652;
    let t7579 = t4991 * t1009;
    let t7580 = t587 * t7579;
    let t7581 = 8.0 / 405.0 * t7580;
    let t7582 = t2815 * t586;
    let t7584 = 8.0 / 45.0 * t7582 * t645;
    let t7585 = t7524 + t7526 - t7529 - t7532 - t7536 - t7538 + t7540 - 2.0 / 27.0 * t7541 + t7563 - t7567 - t7569 - t7572 + 0.33245444444444444444e-1 * t7573 - t7576 + t7578 - t7581 + t7584;
    (t7578, t7581, t7584, t7585)
}
