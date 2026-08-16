//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1117/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1117<F: Float>(t119777: F, t2487: F, t31752: F, t31753: F, t826: F, t231: F, t886: F, t119776: F, t31830: F, t8478: F, t8479: F, t2769: F, t32425: F) -> (F, F, F, F, F, F, F) {
    let t119778 = t119777 * t2487;
    let t119779 = F::cast_from(0.7437465841810202164e-4_f64) * t119778;
    let t119781 = t31752 * t31753 * t826;
    let t119783 = t231 * t886;
    let t119788 = t31830 * t119776;
    let t119789 = t119788 * t2487;
    let t119790 = F::cast_from(0.13223814266738539448e-3_f64) * t119789;
    let t119792 = t8478 * t8479 * t31753;
    let t119808 = t32425 * t2769;
    (t119779, t119781, t119783, t119788, t119790, t119792, t119808)
}
