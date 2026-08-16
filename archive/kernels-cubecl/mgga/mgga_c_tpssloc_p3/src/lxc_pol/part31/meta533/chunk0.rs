//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1747/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1747<F: Float>(t22716: F, t6908: F, t22723: F, t22891: F, t22920: F, t117: F, t5247: F, t6559: F, t22675: F, t22724: F, t6903: F, t22684: F, t6546: F) -> (F, F, F, F, F, F, F) {
    let t80663 = t22716 * t6908;
    let t80670 = t22723 * t22891;
    let t80671 = t80670 * t22920;
    let t80681 = t6559 * t5247 * t117;
    let t80711 = t22724 * t22675;
    let t80722 = t22716 * t6903;
    let t80727 = t6546 * t22684;
    (t80663, t80670, t80671, t80681, t80711, t80722, t80727)
}
