//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 528/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk528<F: Float>(t45: F, t57: F, t2371: F, t508: F, t200: F, t2251: F, t2258: F, t78: F, t202: F, t81: F, t162: F, t187: F, t205: F, t262: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2372 = t508 * t2371;
    let t2375 = F::new(1.0) / t200;
    let t2381 = piecewise3::<F>(t151, F::new(0.0), F::new(4.0) / F::new(9.0) * t2375 * t2251 + F::new(4.0) / F::new(3.0) * t78 * t2258);
    let t2382 = F::new(1.0) / t202;
    let t2388 = piecewise3::<F>(t155, F::new(0.0), F::new(4.0) / F::new(9.0) * t2382 * t2251 - F::new(4.0) / F::new(3.0) * t81 * t2258);
    let t2389 = t2381 + t2388;
    let t2390 = t2389 * t162;
    let t2392 = F::cast_from(0.19751673498613801407e-1_f64) * t2390 * t187;
    let t2393 = t205 * t262;
    (t2372, t2375, t2382, t2389, t2390, t2392, t2393)
}
