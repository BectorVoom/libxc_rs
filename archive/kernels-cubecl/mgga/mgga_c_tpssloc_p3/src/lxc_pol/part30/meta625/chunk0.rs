//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2026/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2026<F: Float>(t252: F, t4119: F, t25160: F, t814: F, t22690: F, t7520: F, t81573: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F) -> (F, F, F, F, F) {
    let t87130 = t252 * t4119;
    let t87135 = t814 * t25160;
    let t87140 = t81573 * t22690 * t7520;
    let t87153 = t6562 * t794 * t25324;
    let t87154 = F::cast_from(0.82246703342411321824e-2_f64) * t87153;
    let t87155 = t23030 * t25258;
    (t87130, t87135, t87140, t87154, t87155)
}
