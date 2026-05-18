//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1151/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1151<F: Float>(t121034: F, t1390: F, t32192: F, t5727: F, t828: F, t8583: F, t5629: F, t8589: F, t121131: F, t125690: F, t121354: F, t33969: F, t8591: F) -> (F, F, F, F) {
    let t125717 = t8583 * t121034 * t32192 * t1390 * t828 * t5727;
    let t125721 = t8583 * t8589 * t32192 * t5629;
    let t125729 = t121131 * t125690;
    let t125732 = t8591 * t121354 * t33969;
    (t125717, t125721, t125729, t125732)
}
