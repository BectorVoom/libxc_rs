//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2385/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2385<F: Float>(t124: F, t2722: F, t10777: F, t10779: F, t2749: F, t2682: F, t820: F, t823: F, t2751: F, t10886: F, t40555: F, t808: F) -> (F, F, F, F, F) {
    let t40583 = t124 * t2722;
    let t40586 = t10777 * t10779 * t40583 * t2749;
    let t40593 = t820 * t823 * t2682;
    let t40594 = t40593 * t2751;
    let t40600 = t10886 * t808 * t40555;
    (t40583, t40586, t40593, t40594, t40600)
}
