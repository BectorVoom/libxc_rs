//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 808/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk808<F: Float>(t169: F, t2628: F, t174: F, t2640: F, t1646: F, t167: F, t2629: F, t160: F, t171: F, t2630: F, t2635: F, t4510: F, t4513: F, t740: F, t829: F, zeta_threshold: F) -> (F, F) {
    let t170 = t169 <= zeta_threshold;
    let t13003 = F::cast_from(1.0_f64) / t2628 / t169;
    let t13014 = F::cast_from(1.0_f64) / t2640 / t174;
    let t13062 = t13003 * t1646;
    let t13065 = t2629 * t167;
    let t13076 = piecewise3::<F>(t170, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13062 * t2630 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13065 * t740 * t829 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4510 * t2635 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t171 * t740 - F::cast_from(8.0_f64) * t4513 * t160);
    (t13014, t13076)
}
