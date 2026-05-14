//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 955/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk955<F: Float>(t40250: F, t40262: F, t43338: F, t46830: F, t46834: F, t46836: F, t46838: F, t46841: F, t46844: F, t46848: F, t46853: F, t46856: F, t46859: F, t46861: F, t46863: F, t46865: F, t46870: F) -> (F,) {
    let t48795 = -0.49658699875514145965e-4 * t40250 + 0.35754263910370185096e-3 * t46830 - 0.85129199786595678799e-5 * t46834 + 0.2553875993597870364e-4 * t46836 - 0.2553875993597870364e-4 * t46838 + t43338 - 0.71845450211182851384e0 * t46841 + 0.17961362552795712846e1 * t46844 + 0.35922725105591425692e0 * t46848 - 0.79453919800822633544e-4 * t40262 - 0.32729593985094410076e0 * t46853 + 0.8182398496273602519e0 * t46856 + 0.16364796992547205038e0 * t46859 - 0.5987120850931904282e-1 * t46861 - 0.5987120850931904282e-1 * t46863 + 0.1489760996265424379e-3 * t46865 + 0.3405167991463827152e-4 * t46870;
    (t48795,)
}
