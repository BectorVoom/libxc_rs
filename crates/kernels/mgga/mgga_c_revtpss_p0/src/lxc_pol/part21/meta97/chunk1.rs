//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 662/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk662<F: Float>(t45: F, t57: F, t190: F, t2258: F, t706: F, t2251: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F) -> (F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2414 = t190 * t2258;
    let t2416 = F::new(4.0) * t706 * t2414;
    let t2422 = piecewise3::<F>(t151, F::new(0.0), -F::new(2.0) / F::new(9.0) * t80 * t2251 + F::new(2.0) / F::new(3.0) * t766 * t2258);
    let t2428 = piecewise3::<F>(t155, F::new(0.0), -F::new(2.0) / F::new(9.0) * t83 * t2251 - F::new(2.0) / F::new(3.0) * t770 * t2258);
    let t2430 = t2422 / F::new(2.0) + t2428 / F::new(2.0);
    (t2414, t2416, t2430)
}
