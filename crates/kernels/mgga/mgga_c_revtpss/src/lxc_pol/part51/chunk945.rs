//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 945/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk945<F: Float>(t121133: F, t3916: F, t121131: F, t121072: F, t2453: F, t32217: F, t25304: F, t32237: F, t136: F, t2457: F, t8585: F, t25898: F, t7063: F, t8578: F, t4104: F, t550: F, t561: F) -> (F, F, F, F, F, F, F, F) {
    let t121134 = t121133 * t3916;
    let t121135 = t121131 * t121134;
    let t121139 = 0.3427046870806409921e-2 * t2453 * t32217 * t121072;
    let t121140 = t25304 * t32237;
    let t121142 = t8585 * t136 * t2457;
    let t121144 = 0.50779446784275991476e-2 * t121140 * t121142;
    let t121146 = t7063 * t8578 * t25898;
    let t121147 = t121146 * t4104;
    let t121165 = t550 * t561;
    (t121134, t121135, t121139, t121142, t121144, t121146, t121147, t121165)
}
