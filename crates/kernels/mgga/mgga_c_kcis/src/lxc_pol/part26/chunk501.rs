//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 501/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk501<F: Float>(t169: F, t174: F, t1646: F, t2629: F, t167: F, t171: F, t740: F, t829: F, t1650: F, t2641: F, t176: F, t833: F, t44: F, t2633: F, zeta_threshold: F) -> (F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t4510 = t2629 * t1646;
    let t4513 = t171 * t167;
    let t4517 = piecewise3::<F>(t170, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4510 * t829 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4513 * t740);
    let t4518 = t2641 * t1650;
    let t4521 = t176 * t167;
    let t4525 = piecewise3::<F>(t175, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4518 * t833 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4521 * t740);
    let t4527 = (t4517 + t4525) * t44;
    let t4532 = F::cast_from(2.0_f64) * t2633;
    (t4510, t4518, t4527, t4532)
}
