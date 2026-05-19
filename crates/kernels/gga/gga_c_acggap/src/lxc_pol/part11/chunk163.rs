//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 163/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk163<F: Float>(t43: F, t50: F, t40: F, t484: F, t483: F, t85: F, t292: F, t474: F, t296: F, t478: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t485 = t40 * t484;
    let t486 = t483 * t85;
    let t487 = F::cast_from(0.19751673498613801407e-1_f64) * t486;
    let t490 = piecewise3::<F>(t44, F::new(0.0), F::new(2.0) / F::new(3.0) * t292 * t474);
    let t493 = piecewise3::<F>(t51, F::new(0.0), F::new(2.0) / F::new(3.0) * t296 * t478);
    let t495 = t490 / F::new(2.0) + t493 / F::new(2.0);
    (t485, t487, t495)
}
