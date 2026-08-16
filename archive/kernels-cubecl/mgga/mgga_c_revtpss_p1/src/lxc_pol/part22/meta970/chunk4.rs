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
    let t61517 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t5864 * t2251 - F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t1491 * t13396 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t18379 * t2258 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t83 * t60717 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4335 * t13312 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5866 * t2251 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t83 * t18281 * t606 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18384 * t2258 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t60754);
    t61517
}
