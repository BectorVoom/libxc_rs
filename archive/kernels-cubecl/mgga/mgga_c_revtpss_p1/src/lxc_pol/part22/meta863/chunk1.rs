//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3016/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3016<F: Float>(t40834: F, t50613: F, t854: F, t14587: F, t2735: F, t40798: F, t826: F, t10777: F, t10779: F, t2749: F, t50412: F, t14686: F, t837: F) -> (F, F, F, F) {
    let t50615 = t40834 * t854 * t50613;
    let t50619 = t2735 * t40798 * t826 * t14587;
    let t50628 = t10777 * t10779 * t50412 * t2749;
    let t50632 = t10777 * t14686 * t50412 * t837;
    (t50615, t50619, t50628, t50632)
}
