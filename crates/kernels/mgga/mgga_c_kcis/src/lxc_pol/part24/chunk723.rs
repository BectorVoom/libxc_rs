//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 723/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk723<F: Float>(t10415: F, t330: F, t1111: F, t3251: F, t1116: F, t2633: F, t1088: F, t3245: F, t977: F, t278: F, t2835: F, t975: F, t119: F, t251: F, t85: F, t361: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10416 = t10415 * t330;
    let t10424 = t3251 * t1111;
    let t10426 = t3251 * t1116;
    let t10443 = 6.0 * t2633;
    let t10450 = t3245 * t1088;
    let t10461 = t977 * t977;
    let t10462 = 1.0 / t10461;
    let t10463 = t278 * t10462;
    let t10466 = t975 * t2835;
    let t10470 = t85 * t119 * t251;
    let t10471 = t10470 * t361;
    (t10416, t10424, t10426, t10443, t10450, t10461, t10462, t10463, t10466, t10470, t10471)
}
