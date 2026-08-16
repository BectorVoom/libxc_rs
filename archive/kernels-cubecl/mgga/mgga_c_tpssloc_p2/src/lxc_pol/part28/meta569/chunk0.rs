//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1848/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1848<F: Float>(t252: F, t4119: F, t22986: F, t6646: F, t829: F, t22690: F, t7520: F, t81573: F, t25249: F, t2684: F, t25324: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t87130 = t252 * t4119;
    let t87133 = t22986 * t6646 * t87130 * t829;
    let t87140 = t81573 * t22690 * t7520;
    let t87150 = t22986 * t6646 * t25249 * t2684;
    let t87153 = t6562 * t794 * t25324;
    (t87130, t87133, t87140, t87150, t87153)
}
