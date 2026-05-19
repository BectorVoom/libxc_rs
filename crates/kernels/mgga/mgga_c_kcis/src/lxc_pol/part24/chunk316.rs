//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 316/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk316<F: Float>(t169: F, t174: F, t1646: F, t171: F, t176: F, t44: F, zeta_threshold: F) -> (F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t1649 = piecewise3::<F>(t170, F::new(0.0), F::new(4.0) / F::new(3.0) * t171 * t1646);
    let t1650 = -t1646;
    let t1653 = piecewise3::<F>(t175, F::new(0.0), F::new(4.0) / F::new(3.0) * t176 * t1650);
    let t1655 = (t1649 + t1653) * t44;
    let t1657 = piecewise3::<F>(t170, F::new(0.0), t1646);
    (t1650, t1655, t1657)
}
