//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 837/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk837<F: Float>(t247: F, t6678: F, t1264: F, t6425: F, t1774: F, t1794: F, t1250: F, t3720: F) -> (F, F, F, F, F) {
    let t6679 = t247 * t6678;
    let t6682 = t1264 * t6425;
    let t6683 = t247 * t6682;
    let t6688 = t1774 * t1794;
    let t6689 = t6688 * t1250;
    let t6690 = t3720 * t6689;
    (t6679, t6683, t6688, t6689, t6690)
}
