//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1069/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1069<F: Float>(t1940: F, t9054: F, t734: F, t2586: F, t7436: F, t1948: F, t11769: F, t9051: F, t17969: F, t7307: F, t22249: F, t740: F, t1950: F, t7337: F, t7406: F, t7320: F, t7327: F) -> (F, F, F, F, F, F, F, F) {
    let t24463 = t9054 * t1940;
    let t24464 = t734 * t24463;
    let t24466 = t2586 * t7436;
    let t24467 = t1948 * t24466;
    let t24469 = t11769 * t9051;
    let t24471 = t17969 * t7307;
    let t24473 = t22249 * t740;
    let t24474 = t24473 * t1950;
    let t24476 = t7337 * t7406;
    let t24478 = t7320 * t7327;
    (t24464, t24466, t24467, t24469, t24471, t24474, t24476, t24478)
}
