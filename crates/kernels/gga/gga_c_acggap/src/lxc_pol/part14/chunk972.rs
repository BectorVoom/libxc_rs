//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 972/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk972<F: Float>(t1426: F, t1579: F, t2297: F, t598: F, t535: F, t8539: F, t1980: F, t38795: F, t7476: F, t2001: F, t5950: F, t1861: F, t7605: F, t1181: F, t604: F, t6192: F, t7426: F) -> (F, F, F, F, F, F) {
    let t39182 = t598 * t1426 * t1579 * t2297;
    let t39186 = t598 * t1426 * t535 * t8539;
    let t39189 = t1980 * t7476 * t38795;
    let t39192 = t2001 * t5950;
    let t39194 = t7605 * t1861;
    let t39203 = t7426 * t1181 * t604 * t6192;
    (t39182, t39186, t39189, t39192, t39194, t39203)
}
