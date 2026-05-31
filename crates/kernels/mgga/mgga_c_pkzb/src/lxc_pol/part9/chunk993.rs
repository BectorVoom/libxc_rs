//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 993/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk993<F: Float>(t24: F, t7930: F, t1003: F, t6097: F, t2179: F, t8: F, t1429: F, t821: F, t1652: F, t1655: F, t3019: F, t3022: F, t6786: F, t82: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t7931 = F::cast_from(0.59793333333333333334e0_f64) * t7930;
    let t7932 = t6097 * t1003;
    let t7935 = t2179 * t8;
    let t7940 = t821 * t1429;
    let t7945 = piecewise3::<F>(t90, F::cast_from(0.0_f64), -F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t7932 * t1652 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t7935 * t6786 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3019 * t1655 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7940 - F::cast_from(2.0_f64) * t3022 * t82);
    (t7931, t7932, t7935, t7940, t7945)
}
