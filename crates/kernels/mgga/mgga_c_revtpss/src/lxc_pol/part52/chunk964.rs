//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 964/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk964<F: Float>(t32776: F, t572: F, t7002: F, t7553: F, t1461: F, t2040: F, t2115: F, t32373: F, t32377: F, t32755: F, t32760: F, t32762: F, t32764: F, t32772: F, t32775: F, t573: F, t7324: F, t7554: F, t7557: F, t8616: F, t8725: F) -> (F, F) {
    let t32778 = 6.0 * t572 * t32776;
    let t32779 = t7553 * t7002;
    let t32781 = 6.0 * t572 * t32779;
    let t32782 = 3.0 * t1461 * t8725 + 6.0 * t2040 * t7554 + 3.0 * t2040 * t7557 + 3.0 * t2115 * t7324 + t32755 * t573 + t32373 + t32377 + t32760 + t32762 + t32764 + t32772 + t32775 + t32778 + t32781 + t8616;
    (t32779, t32782)
}
