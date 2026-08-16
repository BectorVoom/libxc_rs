//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 609/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk609<F: Float>(t169: F, t1646: F, t113: F, t2633: F, t171: F, t2629: F, t1650: F, zeta_threshold: F) -> (F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t6272 = t1646 * t1646;
    let t6276 = F::cast_from(2.0_f64) * t113 + F::cast_from(2.0_f64) * t2633;
    let t6280 = piecewise3::<F>(t170, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2629 * t6272 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t171 * t6276);
    let t6281 = t1650 * t1650;
    (t6272, t6276, t6280, t6281)
}
