//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1346/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1346<F: Float>(t20782: F, t20828: F, t20855: F, t20910: F, t20955: F, t20993: F, t21027: F, t21057: F, t21114: F, t21146: F, t21176: F, t21196: F, t21226: F, t21264: F, t21295: F, t21338: F) -> F {
    let t21342 = t20782 + t20828 + t20855 + t20910 + t20955 + t20993 + t21027 + t21057 + t21114 + t21146 + t21176 + t21196 + t21226 + t21264 + t21295 + t21338;
    t21342
}
