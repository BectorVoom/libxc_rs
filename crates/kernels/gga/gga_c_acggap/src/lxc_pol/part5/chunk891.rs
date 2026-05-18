//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 891/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk891<F: Float>(t1177: F, t13263: F, t839: F, t944: F, t3206: F, t366: F, t374: F, t1145: F, t3570: F, t1117: F, t1121: F, t3573: F) -> (F, F, F, F, F, F, F) {
    let t13264 = t13263 * t1177;
    let t13268 = t944 * t839;
    let t13273 = t3206 * t366;
    let t13274 = t13273 * t374;
    let t13276 = t3570 * t1145;
    let t13278 = t3570 * t1117;
    let t13280 = t3573 * t1121;
    (t13264, t13268, t13273, t13274, t13276, t13278, t13280)
}
