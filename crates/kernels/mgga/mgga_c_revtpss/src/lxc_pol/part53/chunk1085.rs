//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1085/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1085<F: Float>(t119776: F, t31830: F, t2487: F, t31753: F, t8478: F, t8479: F, t2769: F, t32425: F, t31805: F, t31801: F, t2470: F, t31800: F) -> (F, F, F, F, F, F) {
    let t119788 = t31830 * t119776;
    let t119789 = t119788 * t2487;
    let t119792 = t8478 * t8479 * t31753;
    let t119808 = t32425 * t2769;
    let t119809 = t31805 * t119808;
    let t119810 = t119809 * t31801;
    let t119813 = t31800 * t2470;
    (t119788, t119789, t119792, t119808, t119810, t119813)
}
