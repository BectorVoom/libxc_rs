//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 790/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk790<F: Float>(t1810: F, t963: F, t2798: F, t584: F, t1759: F, t1748: F, t2788: F, t1416: F, t959: F, t1831: F, t2747: F, t750: F) -> (F, F, F, F, F, F) {
    let t7664 = t963 * t1810;
    let t7666 = t584 * t2798;
    let t7667 = t7666 * t1759;
    let t7669 = t2788 * t1748;
    let t7671 = t1416 * t959;
    let t7685 = t963 * t1831;
    let t7688 = F::cast_from(0.34631718211362927518e2_f64) * t2747 * t750;
    (t7664, t7667, t7669, t7671, t7685, t7688)
}
