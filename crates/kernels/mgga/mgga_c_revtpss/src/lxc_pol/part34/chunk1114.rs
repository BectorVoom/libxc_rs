//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1114/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1114<F: Float>(t106120: F, t25387: F, t25375: F, t29610: F, t686: F, t72: F, t29668: F, t689: F, t25431: F, t25411: F, t27216: F, t27279: F, t27213: F, t29674: F, t4481: F, t99285: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t106121 = t25387 * t106120;
    let t106123 = t25375 * t106120;
    let t106128 = t29610 * t72 * t686;
    let t106129 = t25387 * t106128;
    let t106150 = t29668 * t689;
    let t106151 = t25431 * t106150;
    let t106153 = t25411 * t106150;
    let t106216 = t27216 * t27279;
    let t106218 = t27213 * t27279;
    let t106235 = t29674 * t689;
    let t106236 = t25431 * t106235;
    let t106238 = t25411 * t106235;
    let t106267 = t99285 * t4481;
    (t106121, t106123, t106128, t106129, t106151, t106153, t106216, t106218, t106236, t106238, t106267)
}
