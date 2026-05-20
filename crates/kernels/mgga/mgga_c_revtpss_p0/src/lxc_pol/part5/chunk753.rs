//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 753/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk753<F: Float>(t1132: F, t5079: F, t1723: F, t3407: F, t1134: F, t1139: F, t1729: F, t698: F) -> (F, F, F, F, F) {
    let t5080 = t1132 * t5079;
    let t5087 = t3407 * t1723;
    let t5088 = t5087 * t1134;
    let t5090 = t1139 * t5079;
    let t5093 = t698 * t1729;
    (t5080, t5087, t5088, t5090, t5093)
}
