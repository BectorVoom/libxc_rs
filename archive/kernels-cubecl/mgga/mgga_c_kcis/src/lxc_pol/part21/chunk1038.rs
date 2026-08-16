//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1038/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1038<F: Float>(t169: F, t740: F, t9323: F, t234: F, t1767: F, t3217: F, t1262: F, t1851: F, t2153: F, t2539: F, t9275: F, t1295: F, t914: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t18374 = F::cast_from(2.0_f64) * t740;
    let t18375 = F::cast_from(6.0_f64) * t9323;
    let t18376 = -t18374 + t18375;
    let t18401 = piecewise3::<F>(t170, F::cast_from(0.0_f64), -t18376);
    let t18402 = t234 * t18401;
    let t19575 = t3217 * t1767;
    let t20572 = t1851 * t1262;
    let t26390 = t2153 * t2539;
    let t26391 = t9275 * t26390;
    let t26392 = F::cast_from(6.0_f64) * t26391;
    let t26393 = t914 * t1295;
    (t18401, t18402, t19575, t20572, t26390, t26391, t26392, t26393)
}
