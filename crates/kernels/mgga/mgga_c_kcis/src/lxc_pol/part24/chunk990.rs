//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 990/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk990<F: Float>(t174: F, t176: F, t18431: F, t20840: F, t20845: F, t2633: F, t4518: F, t833: F, t20839: F, t44: F, t2153: F, t2539: F, t9275: F, zeta_threshold: F) -> (F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t20851 = piecewise3::<f64>(t175, F::new(0.0), -F::new(8.0) / F::new(27.0) * t20840 * t833 - F::new(16.0) / F::new(9.0) * t4518 * t2633 + F::new(4.0) / F::new(9.0) * t20845 * t833 + F::new(4.0) / F::new(3.0) * t176 * t18431);
    let t20853 = (t20839 + t20851) * t44;
    let t26390 = t2153 * t2539;
    let t26391 = t9275 * t26390;
    (t20853, t26390, t26391)
}
