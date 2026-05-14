//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 660/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk660<F: Float>(t1108: F, t721: F, t1833: F, t1883: F, t1962: F, t1967: F, t2730: F, t2741: F, t2755: F, t2760: F, t2766: F, t2768: F, t2772: F, t2776: F, t2780: F) -> (F, F) {
    let t2834 = t1108 * t721;
    let t2848 = -0.1294625e1 * t2755 + 0.258925e1 * t2760 + t1962 - 0.301925e0 * t1833 - 0.301925e0 * t2730 + 0.905775e0 * t2741 + 0.82524375e-1 * t2766 + 0.16504875e0 * t2768 + t1967 - 0.16557e0 * t1883 - 0.16557e0 * t2772 + 0.248355e0 * t2776 + 0.248355e0 * t2780;
    (t2834, t2848)
}
