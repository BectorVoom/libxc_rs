//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 828/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk828<F: Float>(t1088: F, t3245: F, t1014: F, t3171: F, t1008: F, t2811: F, t977: F, t278: F, t2835: F, t975: F, t119: F, t251: F, t85: F) -> (F, F, F, F, F, F, F, F) {
    let t10450 = t3245 * t1088;
    let t10452 = t1014 * t3171;
    let t10454 = t1008 * t2811;
    let t10461 = t977 * t977;
    let t10462 = F::cast_from(1.0_f64) / t10461;
    let t10463 = t278 * t10462;
    let t10466 = t975 * t2835;
    let t10470 = t85 * t119 * t251;
    (t10450, t10452, t10454, t10461, t10462, t10463, t10466, t10470)
}
