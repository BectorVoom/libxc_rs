//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 593/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk593<F: Float>(t3351: F, t7738: F, t511: F, t798: F, t3352: F, t2144: F, t4905: F, t1971: F, t352: F, t495: F, t515: F, t7230: F) -> (F, F, F, F, F, F, F) {
    let t7739 = t3351 * t7738;
    let t7740 = F::cast_from(0.25538759935978703638e-4_f64) * t7739;
    let t7741 = t511 * t798;
    let t7742 = t3352 * t7741;
    let t7743 = t3351 * t7742;
    let t7744 = F::cast_from(0.76616279807936110914e-4_f64) * t7743;
    let t7745 = t2144 * t4905;
    let t7746 = t1971 * t7745;
    let t7747 = t3351 * t7746;
    let t7748 = F::cast_from(0.25538759935978703638e-4_f64) * t7747;
    let t7750 = t515 * t352 * t495;
    let t7751 = t1971 * t7750;
    let t7752 = t7230 * t7751;
    (t7740, t7742, t7744, t7746, t7748, t7751, t7752)
}
