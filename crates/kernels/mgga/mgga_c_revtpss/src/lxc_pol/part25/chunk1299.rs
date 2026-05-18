//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1299/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1299<F: Float>(t3351: F, t890: F, t10818: F, t27763: F, t1113: F, t2832: F, t775: F, t2430: F, t11061: F, t33: F, t2408: F, t10489: F) -> (F, F, F, F, F, F, F, F) {
    let t94276 = t3351 * t890;
    let t94280 = t27763 * t10818;
    let t94286 = t1113 * t2832;
    let t94293 = t3351 * t775;
    let t94297 = t1113 * t2430;
    let t94312 = t33 * t11061;
    let t94316 = t1113 * t2408;
    let t94320 = t33 * t10489;
    (t94276, t94280, t94286, t94293, t94297, t94312, t94316, t94320)
}
