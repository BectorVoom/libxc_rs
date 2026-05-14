//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 615/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk615<F: Float>(t13176: F, t943: F, t10924: F, t2558: F, t9647: F, t10628: F, t5539: F, t10697: F, t3247: F, t10677: F, t883: F, t2562: F, t2765: F, t3358: F, t2787: F, t3338: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13177 = t943 * t13176;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13224 = t883 * t10677;
    let t13225 = t2562 * t13224;
    let t13226 = t943 * t13225;
    let t13250 = t2765 * t3358;
    let t13253 = t2787 * t3338;
    (t13177, t13182, t13183, t13194, t13195, t13200, t13201, t13225, t13226, t13250, t13253)
}
