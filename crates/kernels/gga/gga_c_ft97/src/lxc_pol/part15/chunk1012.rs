//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1012/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1012<F: Float>(t4449: F, t4474: F, t15716: F, t15793: F, t15797: F, t1594: F, t1624: F, t1631: F, t20087: F, t2021: F, t3076: F, t372: F, t374: F, t4467: F, t4491: F, t58911: F, t73718: F, t73912: F, t7914: F, t8042: F, t85414: F, t85618: F, t85630: F, t930: F, t938: F) -> F {
    let t85649 = t4449 * t4474;
    let t85679 = F::cast_from(0.46509801892875584e-1_f64) * t1624 * t374 * t930 * t20087 + F::cast_from(0.23238868087529279928e-2_f64) * t8042 * t1594 * t85649 + F::cast_from(0.77462893625097599764e-3_f64) * t372 * t1594 * t85618 + F::cast_from(0.1116235245429014016e-1_f64) * t1624 * t7914 * t85630 - F::cast_from(0.139529405678626752e0_f64) * t8042 * t374 * t4467 * t4474 + F::cast_from(0.38704743803858356237e-5_f64) * t372 * t2021 * t85414 + F::cast_from(0.81118562704294997116e-3_f64) * t15797 * t58911 - F::cast_from(36.0_f64) * t3076 * t15716 * t4491 + F::cast_from(8.0_f64) * t3076 * t73718 * t938 - F::cast_from(0.81118562704294997116e-3_f64) * t15793 * t73912 + F::cast_from(0.279058811357253504e-1_f64) * t8042 * t1631 * t85649;
    t85679
}
