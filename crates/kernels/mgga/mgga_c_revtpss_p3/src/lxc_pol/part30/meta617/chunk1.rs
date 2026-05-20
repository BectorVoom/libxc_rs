//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2125/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2125<F: Float>(t25082: F, t75353: F, t8717: F, t7311: F, t9593: F, t28196: F, t28198: F, t28166: F, t7234: F, t28168: F, t27153: F, t32113: F) -> (F, F, F, F) {
    let t98574 = F::new(6.0) * t25082 * t8717 * t75353;
    let t98575 = t7311 * t9593;
    let t98578 = F::new(4.0) * t28196 * t98575 * t28198;
    let t98579 = t7234 * t28166;
    let t98581 = F::new(12.0) * t98579 * t28168;
    let t98584 = F::new(6.0) * t25082 * t32113 * t27153;
    (t98574, t98578, t98581, t98584)
}
