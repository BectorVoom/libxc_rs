//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 633/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk633<F: Float>(t261: F, t2826: F, t1100: F, t713: F, t1108: F, t721: F, t1833: F, t1883: F, t1962: F, t1967: F, t2730: F, t2741: F, t2755: F, t2760: F, t2766: F, t2768: F, t2772: F, t2776: F, t2780: F) -> (F, F, F, F) {
    let t2827 = t2826 * t261;
    let t2829 = t1100 * t713;
    let t2834 = t1108 * t721;
    let t2848 = -F::new(0.1294625e1) * t2755 + F::new(0.258925e1) * t2760 + t1962 - F::new(0.301925e0) * t1833 - F::new(0.301925e0) * t2730 + F::new(0.905775e0) * t2741 + F::new(0.82524375e-1) * t2766 + F::new(0.16504875e0) * t2768 + t1967 - F::new(0.16557e0) * t1883 - F::new(0.16557e0) * t2772 + F::new(0.248355e0) * t2776 + F::new(0.248355e0) * t2780;
    (t2827, t2829, t2834, t2848)
}
