//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 589/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk589<F: Float>(t43: F, t2835: F, t1690: F, t2898: F, t1694: F, t817: F, t1281: F, t234: F, t292: F, t5455: F, t822: F, t1699: F, t2910: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t5479 = F::cast_from(0.11696447245269292414e1_f64) * t2835;
    let t5481 = t2898 * t1690;
    let t5486 = t817 * t1694;
    let t5492 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5481 * t234 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1281 * t822 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5486 * t234 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t292 * t5455);
    let t5493 = t2910 * t1699;
    (t5479, t5492, t5493)
}
