//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1955/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1955<F: Float>(t1081: F, t5527: F, t16596: F, t89992: F, t23788: F, t98007: F, t17109: F, t28: F, t25365: F, t98058: F, t25927: F, t98003: F) -> (F, F, F, F, F, F, F) {
    let t100759 = t1081 * t5527;
    let t100766 = t89992 * t16596;
    let t100769 = t23788 * t98007;
    let t100772 = t28 * t17109;
    let t100780 = t89992 * t25365;
    let t100788 = t23788 * t98058;
    let t100791 = t25927 * t98003;
    (t100759, t100766, t100769, t100772, t100780, t100788, t100791)
}
