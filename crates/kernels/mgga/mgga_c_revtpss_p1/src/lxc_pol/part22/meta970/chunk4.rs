//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3242/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3242<F: Float>(t57: F, t13312: F, t13396: F, t1491: F, t18281: F, t18379: F, t18384: F, t2251: F, t2258: F, t4335: F, t5864: F, t5866: F, t606: F, t60717: F, t60754: F, t770: F, t83: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t61517 = piecewise3::<F>(t155, F::new(0.0), -F::new(56.0) / F::new(81.0) * t5864 * t2251 - F::new(32.0) / F::new(27.0) * t1491 * t13396 - F::new(8.0) / F::new(27.0) * t18379 * t2258 - F::new(4.0) / F::new(9.0) * t83 * t60717 - F::new(4.0) / F::new(9.0) * t4335 * t13312 - F::new(8.0) / F::new(27.0) * t5866 * t2251 - F::new(4.0) / F::new(9.0) * t83 * t18281 * t606 - F::new(2.0) / F::new(9.0) * t18384 * t2258 - F::new(2.0) / F::new(3.0) * t770 * t60754);
    t61517
}
