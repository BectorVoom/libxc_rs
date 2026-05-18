//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1049/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1049<F: Float>(t2007: F, t47854: F, t1971: F, t2144: F, t30311: F, t3351: F, t46005: F, t875: F, t7720: F, t9731: F, t674: F, t7715: F, t9734: F) -> (F, F, F, F, F) {
    let t47857 = t47854 * t2007;
    let t47861 = t3351 * t1971 * t2144 * t30311;
    let t47866 = t3351 * t1971 * t875 * t46005;
    let t47868 = t7720 * t9731;
    let t47871 = t9734 * t7715 * t674;
    (t47857, t47861, t47866, t47868, t47871)
}
