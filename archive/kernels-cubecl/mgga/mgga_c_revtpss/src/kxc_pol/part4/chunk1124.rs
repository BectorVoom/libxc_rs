//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1124/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1124<F: Float>(t13620: F, t13622: F, t13623: F, t13624: F, t13629: F, t13631: F, t13633: F, t13634: F, t13635: F, t13636: F, t13637: F, t9394: F, t9415: F, t9421: F, t9427: F, t9546: F) -> F {
    let t13882 = t9394 - t13620 - t13622 + t13623 - t13624 - t13629 + t13631 + t13633 - t13634 + t13635 - t9415 + t9421 + t13636 - t9427 + t13637 + t9546;
    t13882
}
