//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1382/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1382<F: Float>(t57: F, t13312: F, t14413: F, t14416: F, t2251: F, t2258: F, t4384: F, t606: F, t81: F, t14412: F, t162: F, t187: F, t2615: F, t4311: F, zeta_threshold: F) -> (F, F, F) {
    let t155 = t57 <= zeta_threshold;
    let t14424 = piecewise3::<F>(t155, F::new(0.0), F::new(8.0) / F::new(27.0) * t14413 * t2251 + F::new(8.0) / F::new(9.0) * t14416 * t606 + F::new(4.0) / F::new(9.0) * t4384 * t2258 - F::new(4.0) / F::new(3.0) * t81 * t13312);
    let t14425 = t14412 + t14424;
    let t14426 = t14425 * t162;
    let t14428 = F::cast_from(0.19751673498613801407e-1_f64) * t14426 * t187;
    let t14433 = F::new(8.0) * t4311 * t2615;
    (t14425, t14428, t14433)
}
