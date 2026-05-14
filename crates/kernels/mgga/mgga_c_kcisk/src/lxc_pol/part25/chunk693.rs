//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 693/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk693<F: Float>(t7069: F, t719: F, t735: F, t1935: F, t6943: F, t716: F, t740: F, t748: F, t1953: F, t2586: F, t741: F, t1954: F, t2576: F, t1945: F, t2587: F, t2591: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7405 = t719 * t7069;
    let t7406 = t735 * t7405;
    let t7407 = t1935 * t7406;
    let t7409 = t6943 * t716;
    let t7410 = t7409 * t740;
    let t7411 = t7410 * t748;
    let t7413 = t2586 * t1953;
    let t7414 = t741 * t7413;
    let t7416 = t2576 * t1954;
    let t7418 = t1945 * t2587;
    let t7420 = t1945 * t2591;
    (t7405, t7406, t7407, t7409, t7410, t7411, t7413, t7414, t7416, t7418, t7420)
}
