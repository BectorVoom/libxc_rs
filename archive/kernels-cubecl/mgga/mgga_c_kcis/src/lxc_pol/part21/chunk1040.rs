//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1040/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1040<F: Float>(t2169: F, t26406: F, t2533: F, t7630: F, t2161: F, t2770: F, t2153: F, t2626: F, t2538: F, t826: F, t7655: F, t898: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26407 = t2169 * t26406;
    let t26408 = t26407 / F::cast_from(16.0_f64);
    let t26409 = t2533 * t7630;
    let t26410 = F::cast_from(2.0_f64) * t26409;
    let t26411 = t2161 * t2770;
    let t26416 = t2153 * t2626;
    let t26417 = t2538 * t26416;
    let t26418 = F::cast_from(2.0_f64) * t26417;
    let t26419 = t7630 * t826;
    let t26420 = t2538 * t26419;
    let t26421 = F::cast_from(4.0_f64) * t26420;
    let t26422 = t7655 * t898;
    (t26408, t26409, t26410, t26411, t26416, t26417, t26418, t26419, t26420, t26421, t26422)
}
